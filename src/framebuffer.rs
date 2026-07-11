use core::{ffi::c_void, ptr::null, slice};

pub const REQUEST_ID: [u64; 2] = [0x9d5827dcd881dd75, 0xa3148604f6fab11b];

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

    pub const fn response(&self) -> Option<&'static Response> {
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Mask {
    pub size: u8,
    pub shift: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Framebuffer {
    pub address: *mut c_void,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask: Mask,
    pub green_mask: Mask,
    pub blue_mask: Mask,
    pub edid_size: u64,
    pub edid: *mut c_void,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    count: u64,
    framebuffers: *const *const Framebuffer,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub fn len(&self) -> usize {
        self.count.try_into().unwrap()
    }

    pub const fn framebuffers_ptr(&self) -> *const *const Framebuffer {
        self.framebuffers
    }

    pub fn framebuffers(&self) -> Option<&[*const Framebuffer]> {
        Some(unsafe { slice::from_raw_parts(self.framebuffers.as_ref()?, self.len()) })
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.framebuffers().map_or(&[], |x| x))
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}

pub struct Iter<'a> {
    framebuffers: &'a [*const Framebuffer],
    index: usize,
}

impl<'a> Iter<'a> {
    pub const fn new(framebuffers: &'a [*const Framebuffer]) -> Self {
        Self {
            framebuffers,
            index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Framebuffer;

    fn next(&mut self) -> Option<Self::Item> {
        self.framebuffers
            .get(self.index)
            .inspect(|_| self.index += 1)
            .map(|fb| unsafe { (*fb).as_ref().unwrap() })
    }
}
