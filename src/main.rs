#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]
#![feature(ascii_char)]

extern crate alloc;
extern crate core;

pub mod allocator;
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

use allocator::initialize_allocator;
use core::arch::asm;
use fb::{Framebuffer, get_framebuffer_tag};
use fbcli::FramebufferCLI;
use idt::initialize_idt;
use mem::{get_biggest_usable_pool, get_memory_map_tag};
use multiboot::MultibootInfo;
use panic::_hcf;
use pic::{PIC, PIC_DRIVER};

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

    let framebuffer_tag = get_framebuffer_tag(mb2_info);
    if framebuffer_tag.is_none() {
        println!("No framebuffer tag found!");
        _hcf();
    }

    let framebuffer_ = Framebuffer::from_multiboot(framebuffer_tag.unwrap());
    if framebuffer_.is_none() {
        println!("Could not create framebuffer!");
        _hcf();
    }

    let framebuffer = framebuffer_.unwrap();

    let mut fbcli = FramebufferCLI::new(framebuffer);

    fbcli.write_str("Hello world!\r\naaaaaaaaaaaaaaaaaaaaaaaa\r\nbbbbbbbbbbbbbbb\r\ncccccccccccccccccc\r\nasdasdasd");
    fbcli.draw();
    framebuffer.swap();

    loop {
        asm!("hlt")
    }
}
