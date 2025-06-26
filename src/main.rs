#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;

pub mod allocator;
pub mod disk;
pub mod idt;
pub mod io;
pub mod kernel;
pub mod mem;
pub mod multiboot;
pub mod panic;
pub mod pic;
pub mod range;
pub mod vga;

use allocator::initialize_allocator;
use core::arch::asm;
use core::option::Option;
use disk::ATADrive;
use idt::initialize_idt;
use mem::get_biggest_usable_pool;
use multiboot::MultibootInfo;
use panic::_hcf;
use pic::{PIC, PIC_DRIVER};
use range::Range;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    let biggest_usable_memory_pool: Option<Range> = get_biggest_usable_pool(mb2_info);
    if biggest_usable_memory_pool.is_none() {
        println!("No usable memory pools!");
        _hcf();
    }

    initialize_allocator(biggest_usable_memory_pool.unwrap());
    println!("Initialized allocator");

    initialize_idt();
    println!("Initialized IDT");

    {
        let mut lock = PIC_DRIVER.lock();
        *lock = Some(PIC::new());
        let pic = lock.as_mut().unwrap();
        pic.remap(32, 40);

        println!("Initialized PIC");
    }

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
