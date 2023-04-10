use std::{
    ffi::CString,
    fmt::{self, Debug},
    marker::PhantomData,
    ptr::NonNull,
};

use gz_msgs::Message;
use gz_transport_sys as ffi;

use super::Node;

/// A publisher of a topic
pub struct Publisher<T: Message> {
    r#impl: NonNull<ffi::Publisher>,
    buf: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T: Message> Publisher<T> {
    pub(crate) fn new(node: &mut Node, topic: &str) -> Option<Self> {
        let ctopic_name = CString::new(topic).expect("Invalid topic name");
        let ctopic_type = CString::new(T::NAME).expect("Invalid type name");

        Some(Self {
            r#impl: unsafe {
                NonNull::new(ffi::nodeAdvertise(
                    node.raw_mut(),
                    ctopic_name.as_ptr(),
                    ctopic_type.as_ptr(),
                ))?
            },
            buf: Vec::new(),
            _phantom: Default::default(),
        })
    }

    /// Publish a message
    ///
    /// # Examples
    ///
    /// ```
    /// use gz::transport::Node;
    ///
    /// let mut node = Node::new().unwrap();
    /// let mut pub_ = node.advertise::<gz::msgs::StringMsg>("/hello").unwrap();
    /// pub_.publish(&Default::default());
    /// ```
    ///
    /// # Panics
    ///
    /// - If the message cannot be serialized
    pub fn publish(&mut self, msg: &T) -> bool {
        unsafe { self.buf.set_len(0) };
        msg.write_to_vec(&mut self.buf)
            .expect("Failed to serialize message");
        self.buf.push(0);
        unsafe { ffi::publisherPublish(self.r#impl.as_mut(), self.buf.as_ptr() as *const i8) }
    }
}

impl<T: Message> Debug for Publisher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Publisher").finish()
    }
}

impl<T: Message> Drop for Publisher<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::publisherDestroy(&mut self.r#impl.as_ptr());
        }
    }
}
