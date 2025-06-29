use crate::ioport::outb;
use core::fmt::{self, Arguments, Write};
use core::result::Result::Ok;
use spin::Mutex;

pub struct E9;

impl E9 {
    pub const fn new() -> E9 {
        return E9 {};
    }

    pub unsafe fn write(&mut self, chr: u8) {
        outb(0xE9, chr);
    }

    pub unsafe fn write_str(&mut self, string: &str) {
        let str_ptr = string.as_ptr();
        for i in 0..string.len() {
            self.write(*str_ptr.add(i))
        }
    }
}

impl Write for E9 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            self.write_str(s);
        }
        Ok(())
    }
}

pub fn e9_print(args: Arguments<'_>) {
    E9_WRITER.lock().write_fmt(args).unwrap();
}

pub fn e9_println(args: Arguments<'_>) {
    e9_print(args);
    e9_print(format_args!("\r\n"));
}

static E9_WRITER: Mutex<E9> = Mutex::new(E9::new());
