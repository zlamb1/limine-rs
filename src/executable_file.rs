use core::ptr::null;

use crate::file::File;

pub const REQUEST_ID: [u64; 2] = [0xad97e90e83f1ed67, 0x31eb5d1c5ff23b69];

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
    executable_file: *const File,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub const fn executable_file_ptr(&self) -> *const File {
        self.executable_file
    }

    pub const fn executable_file(&self) -> Option<&File> {
        unsafe { self.executable_file.as_ref() }
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}
