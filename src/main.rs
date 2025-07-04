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

pub mod acpi;
pub mod allocator;
pub mod ata;
pub mod disk;
pub mod e9;
pub mod fb;
pub mod fbcli;
pub mod flanterm;
pub mod font;
pub mod idt;
pub mod io;
pub mod ioport;
pub mod kernel;
pub mod mem;
pub mod multiboot;
pub mod panic;
pub mod pic;
pub mod range;
pub mod vga;

use crate::acpi::acpi_init;
use crate::allocator::initialize_allocator;
use crate::fb::{Framebuffer, get_framebuffer_tag};
use crate::fbcli::FramebufferCLI;
use crate::flanterm::{flanterm_fb_init, flanterm_write};
use crate::idt::initialize_idt;
use crate::mem::get_biggest_usable_pool_multiboot;
use crate::multiboot::MultibootInfo;
use crate::pic::{PIC, PIC_DRIVER};
use core::arch::asm;
use core::ffi::c_void;
use core::ptr::null;
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

    if let Some(fb_tag) = get_framebuffer_tag() {
        let ctx = flanterm_fb_init(
            None,
            None,
            (*fb_tag).addr as *mut u32,
            (*fb_tag).width as usize,
            (*fb_tag).height as usize,
            (*fb_tag).pitch as usize,
            (*fb_tag).red_mask_size,
            (*fb_tag).red_field_pos,
            (*fb_tag).green_mask_size,
            (*fb_tag).green_field_pos,
            (*fb_tag).blue_mask_size,
            (*fb_tag).blue_field_pos,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<u32>() as *mut u32,
            null::<c_void>() as *mut c_void,
            0,
            0,
            1,
            0,
            0,
            0,
        );

        flanterm_write(ctx, b"lol".as_ptr(), 3);
    }

    acpi_init();

    println!("Welcome to qos!");

    loop {
        asm!("hlt")
    }
}
