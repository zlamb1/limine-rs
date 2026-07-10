use core::{ptr::null, slice};

use crate::file::File;

pub const REQUEST_ID: [u64; 2] = [0x3e7e279702be32af, 0xca1c4f3bd1280cee];

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
    count: u64,
    modules: *const *const File,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub fn len(&self) -> usize {
        self.count.try_into().unwrap()
    }

    pub const fn modules_ptr(&self) -> *const *const File {
        self.modules
    }

    pub fn modules(&self) -> Option<&[*const File]> {
        Some(unsafe { slice::from_raw_parts(self.modules.as_ref()?, self.len()) })
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.modules().map_or(&[], |x| x))
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}

pub struct Iter<'a> {
    modules: &'a [*const File],
    index: usize,
}

impl<'a> Iter<'a> {
    pub const fn new(modules: &'a [*const File]) -> Self {
        Self { modules, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a File;

    fn next(&mut self) -> Option<Self::Item> {
        self.modules
            .get(self.index)
            .inspect(|_| self.index += 1)
            .map(|module| unsafe { (module).as_ref().unwrap() })
    }
}
