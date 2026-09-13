//! Native echo service used only by transport integration tests.

use std::{
    ffi::{CString, c_char, c_uint, c_void},
    ptr::NonNull,
};

unsafe extern "C" {
    fn testServiceCreate(partition: *const c_char) -> *mut c_void;
    fn testServiceCalls(service: *const c_void) -> c_uint;
    fn testServiceDestroy(service: *mut c_void);
}

pub struct TestService(NonNull<c_void>);

impl TestService {
    pub fn new(partition: &str) -> Self {
        let partition = CString::new(partition).unwrap();
        // SAFETY: partition remains a valid C string throughout construction.
        Self(NonNull::new(unsafe { testServiceCreate(partition.as_ptr()) }).unwrap())
    }

    pub fn calls(&self) -> u32 {
        // SAFETY: this service is live, and its counter is atomic.
        unsafe { testServiceCalls(self.0.as_ptr()) }
    }
}

impl Drop for TestService {
    fn drop(&mut self) {
        // SAFETY: this is the sole owner of the native service.
        unsafe { testServiceDestroy(self.0.as_ptr()) };
    }
}
