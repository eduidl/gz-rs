use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::{self, Debug},
    os::raw::{c_char, c_uint, c_void},
    ptr::NonNull,
    slice,
    time::Duration,
};

use gz_transport_sys as ffi;
use protobuf::Message;

use super::{
    string::{FFIString, StringVec},
    Publisher,
};

type SubCallbackBox = Box<dyn FnMut(*const c_char, usize, *const c_char)>;

unsafe extern "C" fn callback_wrapper(
    data: *const c_char,
    data_size: usize,
    topic_type: *const c_char,
    user_data: *mut c_void,
) {
    let user_data = &mut *(user_data as *mut SubCallbackBox);
    user_data(data, data_size, topic_type);
}

/// A struct that allows a client to communicate with other peers
pub struct Node {
    r#impl: NonNull<ffi::Node>,
    #[allow(clippy::vec_box)]
    callbacks: HashMap<String, Vec<Box<SubCallbackBox>>>,
}

impl Node {
    fn common(ptr: *const c_char) -> Option<Self> {
        Some(Self {
            r#impl: unsafe { NonNull::new(ffi::nodeCreate(ptr))? },
            callbacks: HashMap::new(),
        })
    }

    /// Create a new node
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz::transport::Node;
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
    /// # use gz::transport::Node;
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

    pub(crate) unsafe fn raw_mut(&mut self) -> &mut ffi::Node {
        self.r#impl.as_mut()
    }

    /// Get the list of topics currently advertised in the network
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz::transport::Node;
    /// let node = Node::new().unwrap();
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
    /// # use gz::transport::Node;
    /// let node = Node::new().unwrap();
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
    /// # use gz::transport::Node;
    /// let mut node = Node::new().unwrap();
    /// let pub_ = node.advertise::<gz::msgs::StringMsg>("/hello").unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    pub fn advertise<T>(&mut self, topic: &str) -> Option<Publisher<T>>
    where
        T: Message,
    {
        Publisher::new(self, topic)
    }

    /// Get the list of topics subscribed by this node.
    ///
    /// # Exampless
    ///
    /// ```
    /// # use gz::transport::Node;
    /// let node = Node::new().unwrap();
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

    /// Subscribe to a topic registering a callback
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz::transport::Node;
    /// let mut node = Node::new().unwrap();
    /// node.subscribe::<gz::msgs::StringMsg, _>("/hello", |msg| {
    ///     dbg!(msg);
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    /// - If the topic type name is not a valid ASCII string
    pub fn subscribe<T, F>(&mut self, topic: &str, mut callback: F) -> bool
    where
        T: Message,
        F: FnMut(T) + 'static,
    {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");

        let mut callback = Box::new({
            let topic = topic.to_string();
            let expected_type = CString::new(T::NAME).expect("Invalid type name");

            Box::new(
                move |data: *const c_char, data_size: usize, topic_type: *const c_char| unsafe {
                    let incoming_type = CStr::from_ptr(topic_type);
                    if incoming_type != expected_type.as_c_str() {
                        log::warn!(
                                "Received message from topic '{}' with unexpected type '{}', expected '{}'",
                                &topic,
                                incoming_type.to_str().unwrap(),
                                T::NAME,
                            );
                        return;
                    }

                    match T::parse_from_bytes(slice::from_raw_parts(data as *const u8, data_size)) {
                        Ok(msg) => callback(msg),
                        Err(e) => {
                            log::warn!("Failed to decode message from topic '{}': {}", &topic, e);
                        }
                    };
                },
            ) as Box<dyn FnMut(_, _, _)>
        });

        let ret = unsafe {
            ffi::nodeSubscribe(
                self.r#impl.as_mut(),
                ctopic_name.as_ptr(),
                callback_wrapper,
                callback.as_mut() as *mut _ as *mut c_void,
            )
        };

        if ret {
            self.callbacks
                .entry(topic.into())
                .or_insert_with(Default::default)
                .push(callback);
        }

        ret
    }

    /// Unsubscribe from a topic. If the topic is not currently subscribed, this function does nothing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gz::transport::Node;
    /// let mut node = Node::new().unwrap();
    /// assert!(node.unsubscribe("/hello"));
    /// ```
    ///
    /// # Panics
    ///
    /// - If the topic name is not a valid ASCII string
    pub fn unsubscribe(&mut self, topic: &str) -> bool {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");

        if self.callbacks.get(topic).is_none() {
            log::warn!("No subscribers for topic '{}'", topic);
            return false;
        }

        let _c = self.callbacks.remove(topic);
        unsafe { ffi::nodeUnsubscribe(self.r#impl.as_mut(), ctopic_name.as_ptr()) }
    }

    /// Get the list of services currently advertised in the network
    ///
    /// # Examples
    ///
    /// ```
    /// # use gz::transport::Node;
    /// let node = Node::new().unwrap();
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
    /// # use gz::transport::Node;
    /// let mut node = Node::new().unwrap();
    /// let (res, ok) = node
    ///     .request::<gz::msgs::StringMsg, gz::msgs::StringMsg>(
    ///         "/hello",
    ///         &Default::default(),
    ///         Duration::from_secs(1),
    ///     )
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
        Req: Message,
        Res: Message,
    {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");
        let mut req_serialized = request
            .write_to_bytes()
            .expect("Failed to serialize request");
        req_serialized.push(0);
        let creq_type = CString::new(Req::NAME).expect("Invalid type name");
        let cres_type = CString::new(Res::NAME).expect("Invalid type name");

        let mut res_buf = FFIString::new();
        let mut result = false;
        unsafe {
            if !ffi::nodeRequest(
                self.raw_mut(),
                ctopic_name.as_ptr(),
                req_serialized.as_ptr() as *const c_char,
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
                    log::warn!("Failed to decode response from service '{}': {}", &topic, e);
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
        unsafe { ffi::nodeDestroy(&mut self.r#impl.as_ptr()) };
    }
}
