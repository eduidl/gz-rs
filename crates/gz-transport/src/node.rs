use std::{
    collections::HashSet,
    ffi::{CStr, CString},
    fmt::{self, Debug},
    os::raw::{c_char, c_uint, c_void},
    ptr::NonNull,
    slice,
    time::Duration,
};

use async_channel::{Receiver, bounded};
use gz_msgs_common::GzMessage;
use gz_transport_sys as ffi;

use super::{
    Publisher,
    string::{FFIString, StringVec},
};

type SubCallbackBox = Box<dyn Fn(*const c_char, usize, *const c_char) + Send + Sync>;

unsafe extern "C" fn callback_wrapper(
    data: *const c_char,
    data_size: usize,
    topic_type: *const c_char,
    user_data: *mut c_void,
) {
    // SAFETY: nodeSubscribeOwned retains this boxed callback through every
    // acquired native handler, including concurrent calls after unsubscription.
    let callback = unsafe { &*user_data.cast::<SubCallbackBox>() };
    callback(data, data_size, topic_type);
}

unsafe extern "C" fn destroy_callback(user_data: *mut c_void) {
    // SAFETY: native ownership invokes this exactly once after the last handler
    // releases the pointer transferred by Box::into_raw.
    unsafe { drop(Box::from_raw(user_data.cast::<SubCallbackBox>())) };
}

/// A struct that allows a client to communicate with other peers
pub struct Node {
    r#impl: NonNull<ffi::Node>,
    subscriptions: HashSet<String>,
}

impl Node {
    fn common(ptr: *const c_char) -> Option<Self> {
        Some(Self {
            r#impl: unsafe { NonNull::new(ffi::nodeCreate(ptr))? },
            subscriptions: HashSet::new(),
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
    /// Recommended for sequential state updates, expensive processing, and
    /// choosing your own processing thread. No worker thread is created.
    /// Native callbacks decode and try to enqueue
    /// messages without waiting for the receiver. If the channel is full, the
    /// oldest queued message is dropped with a warning to make room for the new one.
    /// The bound must be at least one.
    /// Panics in your receiving loop are outside Gazebo and handled by your code.
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
    /// while let Ok(msg) = rx.recv_blocking() {
    ///     println!("Received: {:?}", msg.data);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    /// - If `bound` is zero
    pub fn subscribe_channel<T>(&mut self, topic: &str, bound: usize) -> Option<Receiver<T>>
    where
        T: GzMessage,
    {
        let (tx, rx) = bounded(bound);
        let topic_name = topic.to_owned();
        let registered = self.subscribe(topic, move |msg: T| {
            if let Ok(Some(_)) = tx.force_send(msg) {
                log::warn!(
                    "Oldest message from topic '{}' was dropped: subscription queue is full",
                    topic_name
                );
            }
        });
        registered.then_some(rx)
    }

    /// Subscribe with a callback executed directly by Gazebo. Returns false on
    /// registration failure.
    ///
    /// Callbacks may run concurrently or re-enter during local publication.
    /// Keep them short; prefer [`Self::subscribe_channel`] for sequential or
    /// expensive processing. Acquired callbacks may run after unsubscription.
    /// Callbacks and their captures' destructors must not panic: unwinding across
    /// the C ABI boundary aborts the process.
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
    pub fn subscribe<T, F>(&mut self, topic: &str, callback: F) -> bool
    where
        T: GzMessage,
        F: Fn(T) + Send + Sync + 'static,
    {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");
        let callback = {
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
                        Ok(msg) => callback(msg),
                        Err(e) => {
                            log::warn!("Failed to decode message from topic '{}': {}", topic, e);
                        }
                    }
                },
            ) as SubCallbackBox
        };

        // Native ownership consumes the context even when registration fails.
        let user_data = Box::into_raw(Box::new(callback)).cast();
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
            self.subscriptions.insert(topic.into());
        }

        ret
    }

    /// Unsubscribe from a topic. If the topic is not currently subscribed, this function does nothing.
    ///
    /// Does not wait for acquired native callbacks to finish. They retain their
    /// context and may still invoke user callbacks or enqueue channel messages
    /// after this call. Channel receivers disconnect after the last native owner
    /// releases its sender and the remaining messages have been drained.
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

        if !self.subscriptions.contains(topic) {
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
    /// The timeout is truncated to whole milliseconds.
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
    /// - If the timeout in whole milliseconds exceeds [`c_uint::MAX`]; no request is sent
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
        let timeout_ms: c_uint = timeout
            .as_millis()
            .try_into()
            .expect("Timeout in milliseconds exceeds c_uint::MAX");
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
                timeout_ms,
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
