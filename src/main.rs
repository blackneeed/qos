#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;

pub mod allocator;
pub mod disk;
pub mod e9;
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

use allocator::initialize_allocator;
use core::arch::asm;
use disk::ATADrive;
use idt::initialize_idt;
use mem::{get_biggest_usable_pool, get_memory_map_tag};
use multiboot::MultibootInfo;
use panic::_hcf;
use pic::{PIC, PIC_DRIVER};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    initialize_idt();
    println!("Initialized IDT");

    {
        let mut lock = PIC_DRIVER.lock();
        *lock = Some(PIC::new());
        let pic = lock.as_mut().unwrap();
        pic.remap(32, 40);

        println!("Initialized PIC");
    }

    let memory_map_tag = get_memory_map_tag(mb2_info);
    if memory_map_tag.is_none() {
        println!("No memory map tag found!");
        _hcf();
    }

    println!("Found memory map tag");

    let biggest_usable_memory_pool = get_biggest_usable_pool(memory_map_tag.unwrap());
    if biggest_usable_memory_pool.is_none() {
        println!("No usable memory pools!");
        _hcf();
    }

    println!("Found memory pool");

    initialize_allocator(biggest_usable_memory_pool.unwrap());
    println!("Initialized allocator");

    let mut inited = 0u8;
    for id in 0..4 {
        if ATADrive::new(id).is_some() {
            // fine clippy ill use your .is_some()
            inited += 1;
        }
    }

    println!("Initialized {} ATA PIO disks", inited);

    loop {
        asm!("hlt")
    }
}
