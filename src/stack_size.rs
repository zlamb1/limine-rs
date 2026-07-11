use core::ptr::null;

pub const REQUEST_ID: [u64; 2] = [0x224ef0460a8e8926, 0xe1cb0fc25f46ea3d];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Request {
    base: crate::Request,
    response: *const Response,
    stack_size: u64,
}

impl Request {
    pub const fn new(stack_size: u64) -> Self {
        Self {
            base: crate::Request::new(REQUEST_ID[0], REQUEST_ID[1]),
            response: null(),
            stack_size,
        }
    }

    pub const fn base(&self) -> &crate::Request {
        &self.base
    }

    pub const fn response_ptr(&self) -> *const Response {
        self.response
    }

    pub const fn response(&self) -> Option<&'static Response> {
        unsafe { self.response.as_ref() }
    }

    pub const fn stack_size(&self) -> u64 {
        self.stack_size
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
