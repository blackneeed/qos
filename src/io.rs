use crate::e9::{e9_print, e9_println};
use crate::vga::{vga_print, vga_println};
use core::fmt::Arguments;

pub fn print(args: Arguments<'_>) {
    e9_print(args);
    vga_print(args);
}

pub fn println(args: Arguments<'_>) {
    e9_println(args);
    vga_println(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::io::println(format_args!($($arg)*)));
}
