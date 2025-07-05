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
use crate::drv::fb::fbcli::init as fbcli_init;
use crate::drv::io::mm::fb::Framebuffer;
use crate::drv::io::pic::mask_all as pic_mask_all;
use crate::mem::allocator::initialize_allocator;
use crate::mem::pmm::get_biggest_usable_pool_multiboot;
use crate::tables::acpi::acpi_init;
use crate::tables::idt::initialize_idt;
use crate::util::panic::infhlt;
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
            panic!(
                "{}:{}: access of multiboot2 pointer via get_multiboot_info() before initialization",
                file!(),
                line!()
            )
        }
    }
}

fn init_multiboot_info(mb2_info: *const MultibootInfo) {
    *MULTIBOOT_INFO.lock() = Some(SendablePtr {
        ptr: mb2_info as *const (),
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    init_multiboot_info(mb2_info);
    pic_mask_all();
    initialize_idt();
    initialize_allocator(get_biggest_usable_pool_multiboot().expect("no usable memory pools"));
    fbcli_init(Framebuffer::from_multiboot().expect("framebuffer not available"));
    acpi_init();
    infhlt();
}
