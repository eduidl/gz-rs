use std::{
    ffi::{CStr, CString, c_char, c_void},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use gz_transport_sys as ffi;

struct TestNode(*mut ffi::Node);

impl TestNode {
    fn new(name: &str) -> Self {
        let partition =
            CString::new(format!("owned_context_{}_{}", std::process::id(), name)).unwrap();
        // SAFETY: the partition is a valid C string for this call.
        let node = unsafe { ffi::nodeCreate(partition.as_ptr()) };
        assert!(!node.is_null());
        Self(node)
    }

    fn subscribe(&mut self, topic: &CStr, drops: &Arc<AtomicUsize>) -> bool {
        let data = Box::into_raw(Box::new(Arc::clone(drops))).cast();
        // SAFETY: this node is live and data is uniquely transferred to the
        // native owner. The no-op callback is thread safe; destroy_data matches
        // the allocation type and does not unwind.
        unsafe {
            ffi::nodeSubscribeOwned(&mut *self.0, topic.as_ptr(), callback, data, destroy_data)
        }
    }

    fn unsubscribe(&mut self, topic: &CStr) -> bool {
        // SAFETY: this node is live and topic remains valid for the call.
        unsafe { ffi::nodeUnsubscribe(&mut *self.0, topic.as_ptr()) }
    }
}

impl Drop for TestNode {
    fn drop(&mut self) {
        // SAFETY: TestNode is the sole owner of this native node.
        unsafe { ffi::nodeDestroy(&mut self.0) };
    }
}

unsafe extern "C" fn callback(_: *const c_char, _: usize, _: *const c_char, _: *mut c_void) {}

unsafe extern "C" fn destroy_data(data: *mut c_void) {
    // SAFETY: subscribe transferred exactly this Box type. The native owner
    // must invoke the destructor exactly once, as asserted by these tests.
    let drops = unsafe { Box::from_raw(data.cast::<Arc<AtomicUsize>>()) };
    drops.fetch_add(1, Ordering::SeqCst);
}

#[test]
fn registration_failure_releases_owned_data_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut node = TestNode::new("failure");
    assert!(!node.subscribe(c"/invalid topic", &drops));
    assert_eq!(drops.load(Ordering::SeqCst), 1);
    drop(node);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn unsubscribe_releases_each_owned_context_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut node = TestNode::new("unsubscribe");
    assert!(node.subscribe(c"/topic", &drops));
    assert!(node.subscribe(c"/topic", &drops));
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(node.unsubscribe(c"/topic"));
    assert_eq!(drops.load(Ordering::SeqCst), 2);
    drop(node);
    assert_eq!(drops.load(Ordering::SeqCst), 2);
}

#[test]
fn node_destruction_releases_owned_data_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut node = TestNode::new("destruction");
    assert!(node.subscribe(c"/topic", &drops));
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    drop(node);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}
