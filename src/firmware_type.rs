use core::ptr::null;

pub const REQUEST_ID: [u64; 2] = [0x8c2f75d90bef28a8, 0x7045a4688eac00c3];

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum FirmwareType {
    Bios,
    Efi32,
    Efi64,
    Sbi,
}

impl TryFrom<u64> for FirmwareType {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Bios,
            1 => Self::Efi32,
            2 => Self::Efi64,
            3 => Self::Sbi,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    firmware_type: u64,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub const fn firmware_type_raw(&self) -> u64 {
        self.firmware_type
    }

    pub fn firmware_type(&self) -> Option<FirmwareType> {
        self.firmware_type.try_into().ok()
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}
