use crate::drv::fb::tty::{initialized as tty_initialized, tty_print, tty_println};
use crate::drv::io::e9::{e9_print, e9_println};
use core::fmt::Arguments;

const DEBUG: bool = true;
const DEBUG_GLOBAL_EXTRA_KILL_SWITCH: bool = true;
const DEBUG_PRINT_TTY: bool = true;
const DEBUG_PRINT_E9: bool = true;

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

pub fn dprint(file: &'static str, line: u32, print_extra: bool, args: Arguments<'_>) {
    if DEBUG {
        if print_extra && !DEBUG_GLOBAL_EXTRA_KILL_SWITCH {
            if DEBUG_PRINT_TTY {
                tty_print(format_args!("{file}:{line}: "));
            }

            if DEBUG_PRINT_E9 {
                e9_print(format_args!("{file}:{line}: "));
            }
        }

        if DEBUG_PRINT_TTY {
            tty_print(args);
        }

        if DEBUG_PRINT_E9 {
            e9_print(args);
        }
    }
}

pub fn dprintln(file: &'static str, line: u32, print_extra: bool, args: Arguments<'_>) {
    if DEBUG {
        if print_extra && !DEBUG_GLOBAL_EXTRA_KILL_SWITCH {
            if DEBUG_PRINT_TTY {
                tty_print(format_args!("{file}:{line}: "));
            }
            e9_print(format_args!("{file}:{line}: "));
        }
        if DEBUG_PRINT_TTY {
            tty_println(args);
        }

        if DEBUG_PRINT_E9 {
            e9_println(args);
        }
    }
}

pub fn initialized(file: &'static str, line: u32, args: Arguments<'_>) {
    dprint(file, line, true, format_args!("Initialized "));
    dprintln(file, line, false, args);
}

pub fn initialization_fail(file: &'static str, line: u32, args: Arguments<'_>) {
    dprint(file, line, true, format_args!("Failed to initialize "));
    dprintln(file, line, false, args);
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
    ($($arg:tt)*) => ($crate::util::kprint::dprint(file!(), line!(), true, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! dprintln {
    () => ($crate::dprint!("\r\n"));
    ($($arg:tt)*) => ($crate::util::kprint::dprintln(file!(), line!(), true, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! initialized {
    ($($arg:tt)*) => ($crate::util::kprint::initialized(file!(), line!(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! initialization_fail {
    ($($arg:tt)*) => ($crate::util::kprint::initialization_fail(file!(), line!(), format_args!($($arg)*)));
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
