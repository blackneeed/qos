use crate::drv::fb::tty::{initialized as tty_initialized, tty_print, tty_println};
use crate::drv::io::e9::{e9_print, e9_println};
use core::fmt::Arguments;

pub fn kprint(args: Arguments<'_>) {
    e9_print(args);
    if tty_initialized() {
        tty_print(args);
    }
}

pub fn kprintln(args: Arguments<'_>) {
    e9_println(args);
    if tty_initialized() {
        tty_println(args);
    }
}

pub fn dprint(args: Arguments<'_>) {
    e9_print(args);
}

pub fn dprintln(args: Arguments<'_>) {
    e9_println(args);
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

#[macro_export]
macro_rules! dprint {
    ($($arg:tt)*) => ($crate::util::kprint::dprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dprintln {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::util::kprint::dprintln(format_args!($($arg)*)));
}
