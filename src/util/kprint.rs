use crate::drv::fb::fbcli::{fbcli_print, fbcli_println, initialized as fbcli_initialized};
use crate::drv::io::e9::{e9_print, e9_println};
use crate::drv::io::mm::vga::{vga_print, vga_println};
use core::fmt::Arguments;

pub fn kprint(args: Arguments<'_>) {
    e9_print(args);
    if !fbcli_initialized() {
        vga_print(args);
    } else {
        fbcli_print(args);
    }
}

pub fn kprintln(args: Arguments<'_>) {
    e9_println(args);
    if !fbcli_initialized() {
        vga_println(args);
    } else {
        fbcli_println(args);
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::util::kprint::kprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::util::kprint::kprintln(format_args!($($arg)*)));
}
