#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::new_without_default)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
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

use crate::acpi::{FADT, SDT, get_acpi_tag, get_sdt};
use crate::allocator::initialize_allocator;
use crate::fb::{Framebuffer, get_framebuffer_tag};
use crate::fbcli::FramebufferCLI;
use crate::idt::initialize_idt;
use crate::mem::{get_biggest_usable_pool, get_memory_map_tag};
use crate::multiboot::MultibootInfo;
use crate::panic::_hcf;
use crate::pic::{PIC, PIC_DRIVER};
use core::arch::asm;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    {
        let mut lock = PIC_DRIVER.lock();
        *lock = Some(PIC::new());
        let pic = lock.as_mut().unwrap();
        pic.remap(32, 40);
    }

    initialize_idt();

    let memory_map_tag = get_memory_map_tag(mb2_info);
    if memory_map_tag.is_none() {
        println!("No memory map tag found!");
        _hcf();
    }

    let biggest_usable_memory_pool = get_biggest_usable_pool(memory_map_tag.unwrap());
    if biggest_usable_memory_pool.is_none() {
        println!("No usable memory pools!");
        _hcf();
    }

    initialize_allocator(biggest_usable_memory_pool.unwrap());

    let mut fb_i: bool = false;

    if let Some(fbtag) = get_framebuffer_tag(mb2_info)
        && let Some(fb) = Framebuffer::from_multiboot(fbtag)
    {
        fb_i = true;
        fbcli::init(FramebufferCLI::new(fb));
    }

    println!("Initialized:");
    println!("\t- E9");
    println!("\t- VGA");
    if fb_i {
        println!("\t- Framebuffer");
        println!("\t- Framebuffer CLI");
    }

    println!("\t- Allocator");
    println!("Welcome to qos!");

    if let Some(acpi_tag) = get_acpi_tag(mb2_info) {
        if let Some(sdt) = get_sdt(acpi_tag, b"FACP") {
            println!("{:?}", sdt);
            println!(
                "{:?}",
                *(((&raw const *sdt) as *const u8).add(core::mem::size_of::<SDT>()) as *const FADT)
            )
        } else {
            println!("Could not find FADT!");
        }
    } else {
        println!("Could not find ACPI tag!");
    }

    loop {
        asm!("hlt")
    }
}
