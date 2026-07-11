use core::{
    fmt::{Display, Formatter, Result},
    ptr::null,
    slice,
};

pub const REQUEST_ID: [u64; 2] = [0x67cf3d9d378a806f, 0xe304acdfc50c3c62];

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    ExecutableAndModules,
    Framebuffer,
    ReservedMapped,
    Unknown,
}

impl From<u64> for Type {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Usable,
            1 => Self::Reserved,
            2 => Self::AcpiReclaimable,
            3 => Self::AcpiNvs,
            4 => Self::BadMemory,
            5 => Self::BootloaderReclaimable,
            6 => Self::ExecutableAndModules,
            7 => Self::Framebuffer,
            8 => Self::ReservedMapped,
            _ => Self::Unknown,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Usable => f.write_str("usable"),
            Self::Reserved => f.write_str("reserved"),
            Self::AcpiReclaimable => f.write_str("acpi reclaimable"),
            Self::AcpiNvs => f.write_str("acpi nvs"),
            Self::BadMemory => f.write_str("bad memory"),
            Self::BootloaderReclaimable => f.write_str("bootloader reclaimable"),
            Self::ExecutableAndModules => f.write_str("executable and modules"),
            Self::Framebuffer => f.write_str("framebuffer"),
            Self::ReservedMapped => f.write_str("reserved mapped"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Entry {
    base: u64,
    len: u64,
    mem_type: u64,
}

impl Entry {
    pub const fn base(&self) -> u64 {
        self.base
    }

    pub const fn len(&self) -> u64 {
        self.len
    }

    pub const fn mem_type_raw(&self) -> u64 {
        self.mem_type
    }

    pub fn mem_type(&self) -> Type {
        self.mem_type.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    count: u64,
    entries: *const *const Entry,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub fn len(&self) -> usize {
        self.count.try_into().unwrap()
    }

    pub const fn entries_ptr(&self) -> *const *const Entry {
        self.entries
    }

    pub fn entries(&self) -> Option<&[*const Entry]> {
        Some(unsafe { slice::from_raw_parts(self.entries.as_ref()?, self.len()) })
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.entries().map_or(&[], |x| x))
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}

pub struct Iter<'a> {
    entries: &'a [*const Entry],
    index: usize,
}

impl<'a> Iter<'a> {
    pub const fn new(entries: &'a [*const Entry]) -> Self {
        Self { entries, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries
            .get(self.index)
            .inspect(|_| self.index += 1)
            .map(|entry| unsafe { (*entry).as_ref().unwrap() })
    }
}
