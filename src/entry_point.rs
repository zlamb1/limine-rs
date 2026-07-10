use core::ptr::null;

pub const REQUEST_ID: [u64; 2] = [0x13d86c035a1cd3e1, 0x2b0caa89d8f3026a];

pub type EntryPoint = extern "C" fn();

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Request {
    base: crate::Request,
    response: *const Response,
    entry_point: EntryPoint,
}

impl Request {
    pub const fn new(entry_point: EntryPoint) -> Self {
        Self {
            base: crate::Request::new(REQUEST_ID[0], REQUEST_ID[1]),
            response: null(),
            entry_point,
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

    pub const fn entry_point(&self) -> EntryPoint {
        self.entry_point
    }
}

unsafe impl Send for Request {}
unsafe impl Sync for Request {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}
