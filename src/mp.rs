use core::{
    cell::{Cell, UnsafeCell},
    ptr::null,
    slice,
    sync::atomic::{AtomicPtr, Ordering},
};

pub const REQUEST_ID: [u64; 2] = [0x95a67b819a1b857e, 0xa0b61b723b6a73e0];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum Flags {
    #[cfg(target_arch = "x86_64")]
    X2Apic = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Request {
    base: crate::Request,
    response: *const Response,
    flags: u64,
}

impl Request {
    pub const fn new() -> Self {
        Self {
            base: crate::Request::new(REQUEST_ID[0], REQUEST_ID[1]),
            response: null(),
            flags: 0,
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

    pub const fn flags(&self) -> u64 {
        self.flags
    }
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Request {}
unsafe impl Sync for Request {}

pub type GotoAddress = extern "C" fn(cpu: *const Cpu);

#[cfg(target_arch = "x86_64")]
#[derive(Debug)]
#[repr(C)]
pub struct Cpu {
    processor_id: u32,
    lapic_id: u32,
    reserved: u64,
    goto_address: UnsafeCell<*mut ()>,
    extra_argument: Cell<u64>,
}

#[cfg(target_arch = "aarch64")]
#[derive(Debug)]
#[repr(C)]
pub struct Cpu {
    processor_id: u32,
    reserved1: u32,
    mpidr: u64,
    reserved2: u64,
    goto_address: UnsafeCell<*mut ()>,
    extra_argument: Cell<u64>,
}

#[cfg(target_arch = "riscv64")]
#[derive(Debug)]
#[repr(C)]
pub struct Cpu {
    processor_id: u64,
    hartid: u64,
    reserved: u64,
    goto_address: UnsafeCell<*mut ()>,
    extra_argument: Cell<u64>,
}

#[cfg(target_arch = "loongarch64")]
#[derive(Debug)]
#[repr(C)]
pub struct Cpu {
    processor_id: u64,
    phys_id: u64,
    reserved: u64,
    goto_address: UnsafeCell<*mut ()>,
    extra_argument: Cell<u64>,
}

impl Cpu {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    pub const fn processor_id(&self) -> u32 {
        self.processor_id
    }

    #[cfg(any(target_arch = "riscv64", target_arch = "loongarch64"))]
    pub const fn processor_id(&self) -> u64 {
        self.processor_id
    }

    #[cfg(target_arch = "x86_64")]
    pub const fn lapic_id(&self) -> u32 {
        self.lapic_id
    }

    #[cfg(target_arch = "aarch64")]
    pub const fn mpidr(&self) -> u64 {
        self.mpidr
    }

    #[cfg(target_arch = "riscv64")]
    pub const fn hartid(&self) -> u64 {
        self.hartid
    }

    #[cfg(target_arch = "loongarch64")]
    pub const fn phys_id(&self) -> u64 {
        self.phys_id
    }

    pub fn set_goto_address(&self, goto_address: GotoAddress) {
        unsafe {
            let ptr = self.goto_address.get();
            AtomicPtr::from_ptr(ptr).store(goto_address as *mut (), Ordering::Release);
        }
    }

    pub const fn extra_argument(&self) -> u64 {
        self.extra_argument.get()
    }

    pub fn set_extra_argument(&self, extra_argument: u64) {
        self.extra_argument.set(extra_argument);
    }
}

#[cfg(target_arch = "x86_64")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    flags: u32,
    bsp_lapic_id: u32,
    count: u64,
    cpus: *const *const Cpu,
}

#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    flags: u64,
    bsp_mpidr: u64,
    count: u64,
    cpus: *const *const Cpu,
}

#[cfg(target_arch = "riscv64")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    flags: u64,
    bsp_hartid: u64,
    count: u64,
    cpus: *const *const Cpu,
}

#[cfg(target_arch = "loongarch64")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Response {
    rev: u64,
    flags: u64,
    bsp_phys_id: u64,
    count: u64,
    cpus: *const *const Cpu,
}

impl Response {
    pub const fn revision(&self) -> u64 {
        self.rev
    }

    #[cfg(target_arch = "x86_64")]
    pub const fn flags(&self) -> u32 {
        self.flags
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ))]
    pub const fn flags(&self) -> u64 {
        self.flags
    }

    #[cfg(target_arch = "x86_64")]
    pub const fn bsp_lapic_id(&self) -> u32 {
        self.bsp_lapic_id
    }

    #[cfg(target_arch = "aarch64")]
    pub const fn bsp_mpidr(&self) -> u64 {
        self.bsp_mpidr
    }

    #[cfg(target_arch = "riscv64")]
    pub const fn bsp_hartid(&self) -> u64 {
        self.bsp_hartid
    }

    #[cfg(target_arch = "loongarch64")]
    pub const fn bsp_phys_id(&self) -> u64 {
        self.bsp_phys_id
    }

    pub fn len(&self) -> usize {
        self.count.try_into().unwrap()
    }

    pub fn cpus_ptr(&self) -> *const *const Cpu {
        self.cpus
    }

    pub fn cpus(&self) -> Option<&[*const Cpu]> {
        Some(unsafe { slice::from_raw_parts(self.cpus.as_ref()?, self.len()) })
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.cpus().map_or(&[], |x| x))
    }
}

unsafe impl Send for Response {}
unsafe impl Sync for Response {}

pub struct Iter<'a> {
    cpus: &'a [*const Cpu],
    index: usize,
}

impl<'a> Iter<'a> {
    pub const fn new(cpus: &'a [*const Cpu]) -> Self {
        Self { cpus, index: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Cpu;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpus
            .get(self.index)
            .inspect(|_| self.index += 1)
            .map(|cpu| unsafe { (*cpu).as_ref().unwrap() })
    }
}
