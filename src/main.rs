#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![feature(proc_macro_hygiene)]
#![feature(ascii_char)]

extern crate alloc;

pub mod boot;
pub mod drv;
pub mod mem;
pub mod tables;
pub mod util;

use crate::boot::multiboot::MultibootInfo;
use crate::drv::fb::fbcli::{FramebufferCLI, init as fbcli_init};
use crate::drv::io::mm::fb::Framebuffer;
use crate::drv::io::pic::{PIC, PIC_DRIVER};
use crate::mem::allocator::initialize_allocator;
use crate::mem::pmm::get_biggest_usable_pool_multiboot;
use crate::tables::acpi::acpi_init;
use crate::tables::idt::initialize_idt;
use core::arch::asm;
use spin::Mutex;

#[derive(Clone, Copy, Debug)]
pub struct SendablePtr {
    pub ptr: *const (),
}

unsafe impl Send for SendablePtr {}

static MULTIBOOT_INFO: Mutex<Option<SendablePtr>> = Mutex::new(None);

pub fn get_multiboot_info() -> *const MultibootInfo {
    let lock = *MULTIBOOT_INFO.lock();
    match lock {
        Some(x) => x.ptr as *const MultibootInfo,
        None => {
            panic!("Access of multiboot2 pointer via get_multiboot_info() before initialization")
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    *MULTIBOOT_INFO.lock() = Some(SendablePtr {
        ptr: mb2_info as *const (),
    });

    {
        let mut lock = PIC_DRIVER.lock();
        *lock = Some(PIC::new());
        let pic = lock.as_mut().unwrap();
        pic.remap(32, 40);
    }

    initialize_idt();

    let biggest_usable_memory_pool = get_biggest_usable_pool_multiboot();
    if biggest_usable_memory_pool.is_none() {
        panic!("{}:{}: No usable memory pools!", file!(), line!());
    }

    initialize_allocator(biggest_usable_memory_pool.unwrap());

    if let Some(fb) = Framebuffer::from_multiboot() {
        fbcli_init(FramebufferCLI::new(fb));
    }

    acpi_init();

    kprintln!("Welcome to qos!");

    loop {
        asm!("hlt")
    }
}
