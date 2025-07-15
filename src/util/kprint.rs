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

pub fn initialized(args: Arguments<'_>) {
    dprint(format_args!("Initialized "));
    dprintln(args);
}

pub fn initialization_fail(args: Arguments<'_>) {
    dprint(format_args!("Failed to initialize "));
    dprintln(args);
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
    () => ($crate::dprint!("\r\n"));
    ($($arg:tt)*) => ($crate::util::kprint::dprintln(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! initialized {
    ($($arg:tt)*) => ($crate::util::kprint::initialized(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! initialization_fail {
    ($($arg:tt)*) => ($crate::util::kprint::initialization_fail(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! display_for_debug {
    ($type:ty) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
            where
                $type: Debug,
            {
                core::fmt::Debug::fmt(self, f)
            }
        }
    };
}
