#![doc = include_str!("../README.md")]

use std::os::raw::{c_char, c_uint, c_void};

/// wrap `gz::transport::Node`
#[repr(C)]
pub struct Node {
    _private: [u8; 0],
}

/// wrap `gz::transport::Node::Publisher`
#[repr(C)]
pub struct Publisher {
    _private: [u8; 0],
}

/// wrap `std::string`
#[repr(C)]
pub struct String {
    _private: [u8; 0],
}

/// warp `std::vector<std::string>`
#[repr(C)]
pub struct StringVec {
    _private: [u8; 0],
}

pub type SubscriberCallback =
    unsafe extern "C" fn(*const c_char, usize, *const c_char, *mut c_void);

/// Releases the user data owned by a native subscription.
pub type SubscriberDataDestructor = unsafe extern "C" fn(*mut c_void);

unsafe extern "C" {
    // Node
    pub fn nodeCreate(partition: *const c_char) -> *mut Node;
    pub fn nodeDestroy(node: &mut *mut Node);

    // Topic
    pub fn nodeTopicList(node: &Node) -> *mut StringVec;

    // Topci Pub
    pub fn nodeAdvertisedTopics(node: &Node) -> *mut StringVec;
    pub fn nodeAdvertise(
        node: &mut Node,
        topic: *const c_char,
        topic_type: *const c_char,
    ) -> *mut Publisher;
    pub fn publisherPublish(
        publisher: &mut Publisher,
        data: *const c_char,
        data_len: usize,
    ) -> bool;
    pub fn publisherDestroy(publisher: &mut *mut Publisher);

    // Topic Sub
    pub fn nodeSubscribedTopics(node: &Node) -> *mut StringVec;
    /// Registers a callback with borrowed user data.
    ///
    /// # Safety
    /// The callback and user data must support concurrent native calls. The
    /// caller must retain the data until all acquired native handlers finish;
    /// unsubscription or Node destruction alone does not establish this.
    /// Prefer [`nodeSubscribeOwned`] for native-managed ownership.
    pub fn nodeSubscribe(
        node: &mut Node,
        topic: *const c_char,
        callback: SubscriberCallback,
        user_data: *mut c_void,
    ) -> bool;
    /// Registers a callback and takes ownership of its user data, even on failure.
    ///
    /// `destroy_user_data` is called exactly once after the last native callback
    /// owner releases the data. This may happen before this function returns,
    /// or on another thread after unsubscription or Node destruction.
    ///
    /// # Safety
    /// `node` must be live, and `topic` must point to a valid NUL-terminated
    /// string for this call. `user_data` must be valid for both callbacks and
    /// must not be freed by the caller after ownership is transferred. The
    /// callback must support concurrent calls. Neither callback may unwind;
    /// the destructor must be valid on any thread and must release the data.
    pub fn nodeSubscribeOwned(
        node: &mut Node,
        topic: *const c_char,
        callback: SubscriberCallback,
        user_data: *mut c_void,
        destroy_user_data: SubscriberDataDestructor,
    ) -> bool;
    pub fn nodeUnsubscribe(node: &mut Node, topic: *const c_char) -> bool;

    // Service
    pub fn nodeServiceList(node: &Node) -> *mut StringVec;

    // Service Client
    pub fn nodeRequest(
        node: &mut Node,
        topic: *const c_char,
        req: *const c_char,
        req_len: usize,
        reqtype: *const c_char,
        restype: *const c_char,
        timeout: c_uint,
        res: &mut String,
        result: &mut bool,
    ) -> bool;

    pub fn stringCreate() -> *mut String;
    pub fn stringLength(string: &String) -> usize;
    pub fn stringGet(string: &String) -> *const c_char;
    pub fn stringDestroy(string: &mut *mut String);

    pub fn stringVecCreate() -> *mut StringVec;
    pub fn stringVecSize(stringVec: &StringVec) -> usize;
    pub fn stringVecAt(stringVec: &StringVec, index: usize) -> *const c_char;
    pub fn stringVecDestroy(stringVec: &mut *mut StringVec);

    pub fn waitForShutdown();
}
