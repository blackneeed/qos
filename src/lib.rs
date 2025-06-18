#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

pub mod io;
pub mod vga;
pub mod multiboot;
pub mod range;
pub mod idt;
pub mod panic;
pub mod kernel;
pub mod mem;
pub mod pic;
pub mod disk;