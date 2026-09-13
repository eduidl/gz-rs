use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::{self, Debug},
    os::raw::{c_char, c_uint, c_void},
    ptr::NonNull,
    slice, thread,
    time::Duration,
};

use crossbeam_channel::{Receiver, Sender, TrySendError, bounded, select_biased};
use gz_msgs_common::GzMessage;
use gz_transport_sys as ffi;

use super::{
    Publisher,
    string::{FFIString, StringVec},
};

const CALLBACK_QUEUE_CAPACITY: usize = 1024;

type SubCallbackBox = Box<dyn Fn(*const c_char, usize, *const c_char) + Send + Sync>;

struct Subscription {
    // Native callbacks own their context independently. Rust only keeps the
    // worker's stop sender so cancellation does not wait for native handlers.
    _stop: Option<Sender<()>>,
}

unsafe extern "C" fn callback_wrapper(
    data: *const c_char,
    data_size: usize,
    topic_type: *const c_char,
    user_data: *mut c_void,
) {
    // SAFETY: nodeSubscribeOwned keeps this boxed callback alive through every
    // acquired native handler, including calls after unsubscription. Concurrent
    // callers share a Fn + Send + Sync that only decodes and enqueues messages.
    let callback = unsafe { &*user_data.cast::<SubCallbackBox>() };
    callback(data, data_size, topic_type);
}

unsafe extern "C" fn destroy_callback(user_data: *mut c_void) {
    // SAFETY: this pointer comes from Box::into_raw in register_subscription.
    // The native shared owner invokes this once, after all handlers release it.
    unsafe { drop(Box::from_raw(user_data.cast::<SubCallbackBox>())) };
}

/// A struct that allows a client to communicate with other peers
pub struct Node {
    r#impl: NonNull<ffi::Node>,
    subscriptions: HashMap<String, Vec<Subscription>>,
}

impl Node {
    fn common(ptr: *const c_char) -> Option<Self> {
        Some(Self {
            r#impl: unsafe { NonNull::new(ffi::nodeCreate(ptr))? },
            subscriptions: HashMap::new(),
        })
    }

    /// Create a new node
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// let node = Node::new().unwrap();
    /// ```
    pub fn new() -> Option<Self> {
        Self::common(std::ptr::null())
    }

    /// Crate a new node with a specific partition name
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// let node = Node::with_partition("ns1").unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// - If the partition name is not a valid ASCII string
    pub fn with_partition(partition: &str) -> Option<Self> {
        let cstr = CString::new(partition).expect("Invalid partition name");
        Self::common(cstr.as_ptr())
    }

    pub(crate) const unsafe fn raw_mut(&mut self) -> &mut ffi::Node {
        unsafe { self.r#impl.as_mut() }
    }

    /// Get the list of topics currently advertised in the network.
    ///
    /// Depending on the Gazebo Transport version, this may also include topics
    /// subscribed to within the current process, even if they have no publisher.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// # let node = Node::new().unwrap();
    /// for topic in node.topic_list() {
    ///     println!("{}", topic);
    /// }
    /// ```
    pub fn topic_list(&self) -> Vec<String> {
        unsafe { StringVec::from_raw(ffi::nodeTopicList(self.r#impl.as_ref())) }
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Get the list of topics advertised by this node.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// # let node = Node::new().unwrap();
    /// for topic in node.advertised_topics() {
    ///     println!("{}", topic);
    /// }
    /// ```
    pub fn advertised_topics(&self) -> Vec<String> {
        unsafe { StringVec::from_raw(ffi::nodeAdvertisedTopics(self.r#impl.as_ref())) }
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Advertise a new topic. If a topic is currently advertised, you cannot advertise it a second time (regardless of its type)
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// use gz_msgs::stringmsg::StringMsg;
    ///
    /// # let mut node = Node::new().unwrap();
    /// let publisher = node.advertise::<StringMsg>("topic_name").unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    pub fn advertise<T>(&mut self, topic: &str) -> Option<Publisher<T>>
    where
        T: GzMessage,
    {
        Publisher::new(self, topic)
    }

    /// Get the list of topics subscribed by this node.
    ///
    /// # Exampless
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// # let node = Node::new().unwrap();
    /// for topic in node.subscribed_topics() {
    ///     println!("{}", topic);
    /// }
    /// ```
    pub fn subscribed_topics(&self) -> Vec<String> {
        unsafe { StringVec::from_raw(ffi::nodeSubscribedTopics(self.r#impl.as_ref())) }
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Subscribe to a topic and receive owned messages through a bounded channel.
    ///
    /// No worker thread is created. Native callbacks decode and try to enqueue
    /// messages without waiting for the receiver. If the channel is full, the
    /// incoming message is dropped with a warning; queued messages are retained.
    /// A zero bound only delivers when a receiver is already waiting.
    ///
    /// Native callbacks already acquired by Gazebo may still enqueue messages
    /// after successful unsubscription or Node destruction. The channel
    /// disconnects once those callbacks finish and queued messages are drained.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gz_transport::Node;
    /// use gz_msgs::stringmsg::StringMsg;
    ///
    /// # let mut node = Node::new().unwrap();
    /// let rx = node
    ///     .subscribe_channel::<StringMsg>("topic_name", 10)
    ///     .unwrap();
    ///
    /// for msg in rx {
    ///     println!("Received: {:?}", msg.data);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    pub fn subscribe_channel<T>(&mut self, topic: &str, bound: usize) -> Option<Receiver<T>>
    where
        T: GzMessage,
    {
        let (tx, rx) = bounded(bound);
        self.register_subscription(topic, tx, None).then_some(rx)
    }

    /// Subscribe to a topic registering a callback
    ///
    /// Each subscription owns a worker thread and a queue of 1024 messages.
    /// The worker exclusively owns the [`Send`] callback and calls it in queue
    /// order, so it need not be [`Sync`]. Native callbacks only decode and enqueue
    /// messages. When the queue is full, the incoming message is dropped with a
    /// warning. Use [`Self::subscribe_channel`] to choose the queue capacity and
    /// run message processing on your own thread instead.
    ///
    /// Execution is asynchronous, including for local publishers: publishing
    /// does not wait for the user callback. Publishing back to the same topic
    /// enqueues another message instead of re-entering the callback. Do not wait
    /// inside the callback for a later invocation of the same callback.
    ///
    /// Successful unsubscription or Node destruction requests worker shutdown
    /// without waiting for user code. An already dispatched callback may still
    /// run; the worker discards pending messages and exits afterward. A callback
    /// panic terminates its worker, without unwinding through Gazebo.
    /// Returns false if native registration or worker creation fails.
    ///
    /// Capturing non-`Send` state is rejected:
    ///
    /// ```compile_fail,E0277
    /// use std::{cell::Cell, rc::Rc};
    /// use gz_msgs::stringmsg::StringMsg;
    /// use gz_transport::Node;
    ///
    /// let mut node = Node::new().unwrap();
    /// let count = Rc::new(Cell::new(0));
    /// assert!(node.subscribe("topic_name", move |_: StringMsg| {
    ///     count.set(count.get() + 1);
    /// }));
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// use gz_msgs::stringmsg::StringMsg;
    ///
    /// # let mut node = Node::new().unwrap();
    /// assert!(node.subscribe("topic_name", |msg: StringMsg| {
    ///     dbg!(msg);
    /// }));
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    #[must_use]
    pub fn subscribe<T, F>(&mut self, topic: &str, mut callback: F) -> bool
    where
        T: GzMessage,
        F: FnMut(T) + Send + 'static,
    {
        let (tx, rx) = bounded(CALLBACK_QUEUE_CAPACITY);
        let (stop_tx, stop_rx) = bounded::<()>(0);
        let worker = thread::Builder::new()
            .name("gz-subscription".into())
            .spawn(move || {
                loop {
                    select_biased! {
                        recv(stop_rx) -> _ => break,
                        recv(rx) -> msg => match msg {
                            Ok(msg) => callback(msg),
                            Err(_) => break,
                        },
                    }
                }
            });
        match worker {
            Ok(worker) => {
                // Detach: shutdown must not wait for arbitrary user code.
                drop(worker);
                self.register_subscription(topic, tx, Some(stop_tx))
            }
            Err(e) => {
                log::warn!(
                    "Failed to start subscription worker for topic '{}': {}",
                    topic,
                    e
                );
                false
            }
        }
    }

    fn register_subscription<T>(
        &mut self,
        topic: &str,
        tx: Sender<T>,
        stop: Option<Sender<()>>,
    ) -> bool
    where
        T: GzMessage,
    {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");
        let callback = Box::new({
            let topic = topic.to_string();
            let expected_type = CString::new(T::GZ_TYPE_NAME).expect("Invalid type name");

            Box::new(
                move |data: *const c_char, data_size: usize, topic_type: *const c_char| unsafe {
                    let incoming_type = CStr::from_ptr(topic_type);
                    if incoming_type != expected_type.as_c_str() {
                        log::warn!(
                            "Received message from topic '{}' with unexpected type '{}', expected '{}'",
                            topic,
                            incoming_type.to_string_lossy(),
                            T::GZ_TYPE_NAME,
                        );
                        return;
                    }

                    match T::parse_from_bytes(slice::from_raw_parts(data.cast::<u8>(), data_size)) {
                        Ok(msg) => match tx.try_send(msg) {
                            Ok(()) | Err(TrySendError::Disconnected(_)) => {}
                            Err(TrySendError::Full(_)) => {
                                log::warn!(
                                    "Incoming message from topic '{}' was dropped: subscription queue is full",
                                    topic
                                );
                            }
                        },
                        Err(e) => {
                            log::warn!("Failed to decode message from topic '{}': {}", topic, e);
                        }
                    }
                },
            ) as SubCallbackBox
        });

        // Transfer the stable outer allocation to the native callback owner.
        // The FFI consumes it on both success and failure, so Rust must not
        // reconstruct or retain a Box after the call.
        let user_data = Box::into_raw(callback).cast();
        let ret = unsafe {
            ffi::nodeSubscribeOwned(
                self.r#impl.as_mut(),
                ctopic_name.as_ptr(),
                callback_wrapper,
                user_data,
                destroy_callback,
            )
        };

        if ret {
            self.subscriptions
                .entry(topic.into())
                .or_default()
                .push(Subscription { _stop: stop });
        }

        ret
    }

    /// Unsubscribe from a topic. If the topic is not currently subscribed, this function does nothing.
    ///
    /// On success, callback workers are asked to stop without waiting for an
    /// already dispatched callback to finish. Pending worker messages are
    /// discarded. Already acquired native callbacks retain their context and
    /// may still enqueue messages for [`Self::subscribe_channel`] receivers.
    /// Those receivers disconnect after the callbacks finish and the queue is
    /// drained.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gz_transport::Node;
    /// # let mut node = Node::new().unwrap();
    /// assert!(node.unsubscribe("topic_name"));
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    #[must_use]
    pub fn unsubscribe(&mut self, topic: &str) -> bool {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");

        if !self.subscriptions.contains_key(topic) {
            log::warn!("No subscribers for topic '{}'", topic);
            return false;
        }

        let ret = unsafe { ffi::nodeUnsubscribe(self.r#impl.as_mut(), ctopic_name.as_ptr()) };

        if ret {
            self.subscriptions.remove(topic);
        }

        ret
    }

    /// Get the list of services currently advertised in the network
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz_transport::Node;
    /// #
    /// # let node = Node::new().unwrap();
    /// for service in node.service_list() {
    ///     println!("{}", service);
    /// }
    /// ```
    pub fn service_list(&self) -> Vec<String> {
        unsafe { StringVec::from_raw(ffi::nodeServiceList(self.r#impl.as_ref())) }
            .unwrap()
            .try_into()
            .unwrap()
    }

    /// Request a new service using a blocking call.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// # use gz_transport::Node;
    /// # use gz_msgs::stringmsg::StringMsg;
    /// #
    /// # let mut node = Node::new().unwrap();
    /// let (res, ok) = node
    ///     .request::<StringMsg, StringMsg>("service", &Default::default(), Duration::from_secs(1))
    ///     .unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the request type name is not a valid ASCII string
    /// - If the response type name is not a valid ASCII string
    pub fn request<Req, Res>(
        &mut self,
        topic: &str,
        request: &Req,
        timeout: Duration,
    ) -> Option<(Res, bool)>
    where
        Req: GzMessage,
        Res: GzMessage,
    {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");
        let req_serialized = request
            .write_to_bytes()
            .expect("Failed to serialize request");
        let creq_type = CString::new(Req::GZ_TYPE_NAME).expect("Invalid type name");
        let cres_type = CString::new(Res::GZ_TYPE_NAME).expect("Invalid type name");

        let mut res_buf = FFIString::new();
        let mut result = false;
        unsafe {
            if !ffi::nodeRequest(
                self.raw_mut(),
                ctopic_name.as_ptr(),
                req_serialized.as_ptr() as *const c_char,
                req_serialized.len(),
                creq_type.as_ptr(),
                cres_type.as_ptr(),
                timeout.as_millis() as c_uint,
                res_buf.raw_mut(),
                &mut result,
            ) {
                return None;
            }

            match Res::parse_from_bytes(slice::from_raw_parts(
                res_buf.as_ptr() as *const u8,
                res_buf.len(),
            )) {
                Ok(res) => Some((res, result)),
                Err(e) => {
                    log::warn!("Failed to decode response from service '{}': {}", topic, e);
                    None
                }
            }
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node").finish()
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.subscriptions.clear();
        unsafe { ffi::nodeDestroy(&mut self.r#impl.as_ptr()) };
    }
}
