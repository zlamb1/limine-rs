use core::{ffi::CStr, ffi::c_char, ptr::null};

use crate::utility::as_cstr;

pub const REQUEST_ID: [u64; 2] = [0x4b161536e598651e, 0xb390ad4a2f1f303a];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Request {
    base: crate::Request,
    response: *const Response,
}

impl Request {
    pub const fn new() -> Self {
        Self {
            base: crate::Request::new(REQUEST_ID[0], REQUEST_ID[1]),
            response: null(),
        }
    }

    pub const fn base(&self) -> &crate::Request {
        &self.base
    }

    pub const fn response_ptr(&self) -> *const Response {
        self.response
    }

    pub const fn response(&self) -> Option<&Response> {
        unsafe { self.response.as_ref() }
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Request {}
unsafe impl Sync for Request {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    cmdline: *const c_char,
}

impl Response {
    pub fn revision(&self) -> u64 {
        self.rev
    }

    pub fn cmdline_ptr(&self) -> *const c_char {
        self.cmdline
    }

    pub fn cmdline(&self) -> Option<&CStr> {
        unsafe { as_cstr(self.cmdline) }
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}
