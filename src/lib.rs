#![cfg_attr(not(feature = "std"), no_std)]

pub mod bootloader_info;
pub mod date_at_boot;
pub mod device_tree;
pub mod entry_point;
pub mod executable_cmdline;
pub mod executable_file;
pub mod file;
pub mod firmware_type;
pub mod framebuffer;
pub mod hhdm;
pub mod memmap;
pub mod module;
pub mod mp;
pub mod rsdp;
pub mod stack_size;
pub mod tsc_frequency;

mod utility;

pub const COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];

#[macro_export]
macro_rules! requests_start_marker {
    () => {
        0xf6b8f4b39de7d1ae, 0xfab91a6940fcb9cf, 0x785c6ed015d3e316, 0x181e920a7852b9d9
    };
}

#[macro_export]
macro_rules! requests_end_marker {
    () => {0xadc0e0531bb10d03, 0x9572709f31764c62};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Request {
    id: [u64; 4],
    rev: u64,
}

impl Request {
    const fn new(m0: u64, m1: u64) -> Self {
        Self {
            id: [COMMON_MAGIC[0], COMMON_MAGIC[1], m0, m1],
            rev: 0,
        }
    }

    pub const fn id(&self) -> [u64; 4] {
        self.id
    }

    pub const fn revision(&self) -> u64 {
        self.rev
    }
}
