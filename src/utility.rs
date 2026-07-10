use core::ffi::{CStr, c_char};

pub const unsafe fn as_cstr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if !ptr.is_null() {
        unsafe { Some(CStr::from_ptr(ptr)) }
    } else {
        None
    }
}
