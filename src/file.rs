use core::ffi::{CStr, c_char, c_void};

use crate::utility::as_cstr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct UUID {
    pub a: u32,
    pub b: u16,
    pub c: u16,
    pub d: [u8; 8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct File {
    rev: u64,
    /// 4 KiB aligned
    address: *mut c_void,
    len: u64,
    path: *const c_char,
    string: *const c_char,
    media_type: u32,
    unused: u32,
    tftp_ipv4: [u8; 4],
    tftp_port: u32,
    partition_index: u32,
    mbr_disk_id: u32,
    gpt_disk_uuid: UUID,
    gpt_part_uuid: UUID,
    part_uuid: UUID,
}

impl File {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    pub const fn address(&self) -> *mut c_void {
        self.address
    }

    pub const fn len(&self) -> u64 {
        self.len
    }

    pub const fn path_ptr(&self) -> *const c_char {
        self.path
    }

    pub const fn path(&self) -> Option<&CStr> {
        unsafe { as_cstr(self.path) }
    }

    pub const fn string_ptr(&self) -> *const c_char {
        self.string
    }

    pub const fn string(&self) -> Option<&CStr> {
        unsafe { as_cstr(self.string) }
    }

    pub const fn media_type(&self) -> u32 {
        self.media_type
    }

    pub const fn tftp_ipv4(&self) -> [u8; 4] {
        self.tftp_ipv4
    }

    pub const fn tftp_port(&self) -> u32 {
        self.tftp_port
    }

    pub const fn partition_index(&self) -> u32 {
        self.partition_index
    }

    pub const fn mbr_disk_id(&self) -> u32 {
        self.mbr_disk_id
    }

    pub const fn gpt_disk_uuid(&self) -> UUID {
        self.gpt_disk_uuid
    }

    pub const fn gpt_part_uuid(&self) -> UUID {
        self.gpt_part_uuid
    }

    pub const fn part_uuid(&self) -> UUID {
        self.part_uuid
    }
}
