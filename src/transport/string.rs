use std::{ffi::CStr, os::raw::c_char, ptr::NonNull, str};

use gz_transport_sys as ffi;

#[derive(Debug)]
pub struct FFIString {
    r#impl: NonNull<ffi::String>,
}

impl FFIString {
    pub fn new() -> Self {
        Self {
            r#impl: unsafe { NonNull::new(ffi::stringCreate()).unwrap() },
        }
    }

    pub(crate) unsafe fn raw_mut(&mut self) -> &mut ffi::String {
        self.r#impl.as_mut()
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::stringLength(self.r#impl.as_ref()) }
    }

    pub(crate) unsafe fn as_ptr(&self) -> *const c_char {
        ffi::stringGet(self.r#impl.as_ref())
    }
}

impl Drop for FFIString {
    fn drop(&mut self) {
        unsafe {
            ffi::stringDestroy(&mut self.r#impl.as_ptr());
        }
    }
}

#[derive(Debug)]
pub struct StringVec {
    r#impl: NonNull<ffi::StringVec>,
}

impl StringVec {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) unsafe fn from_raw(r#impl: *mut ffi::StringVec) -> Option<Self> {
        Some(Self {
            r#impl: NonNull::new(r#impl)?,
        })
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::stringVecSize(self.r#impl.as_ref()) }
    }

    pub fn get(&self, index: usize) -> Result<&str, str::Utf8Error> {
        unsafe { CStr::from_ptr(ffi::stringVecAt(self.r#impl.as_ref(), index)).to_str() }
    }
}

impl Default for StringVec {
    fn default() -> Self {
        Self {
            r#impl: unsafe { NonNull::new(ffi::stringVecCreate()).unwrap() },
        }
    }
}

impl TryFrom<StringVec> for Vec<String> {
    type Error = str::Utf8Error;

    fn try_from(vec: StringVec) -> Result<Self, Self::Error> {
        (0..vec.len())
            .map(|i| vec.get(i).map(|v| v.to_string()))
            .collect()
    }
}

impl Drop for StringVec {
    fn drop(&mut self) {
        unsafe { ffi::stringVecDestroy(&mut self.r#impl.as_ptr()) };
    }
}
