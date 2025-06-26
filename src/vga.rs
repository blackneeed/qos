use crate::io::outb;
use core::fmt::{self, Arguments, Write};
use core::format_args;
use core::marker::Send;
use core::result::Result::Ok;
use spin::Mutex;

pub struct VGA {
    vga_ptr: *mut u8,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl VGA {
    pub const fn new() -> VGA {
        return VGA {
            vga_ptr: 0xb8000 as *mut u8,
            x: 0,
            y: 0,
            w: 80,
            h: 25,
        };
    }

    unsafe fn get_ptr(&mut self) -> *mut u8 {
        return self.vga_ptr.add(((self.y * self.w + self.x) * 2) as usize);
    }

    unsafe fn _write(&mut self, chr: u8) {
        *self.get_ptr() = chr;
        *self.get_ptr().add(1) = 0x0F;
        self.next_char();
        let abs_pos: u16 = self.y * self.w + self.x;
        outb(0x3D4, 0xE);
        outb(0x3D5, (abs_pos >> 8) as u8);
        outb(0x3D4, 0xF);
        outb(0x3D5, abs_pos as u8);
    }

    pub unsafe fn write(&mut self, chr: u8) {
        match chr {
            b'\r' => {
                self.x = 0;
            }
            b'\n' => {
                self.next_line();
            }
            b'\t' => {
                for _ in 0..4 {
                    self._write(b' ');
                }
            }
            _ => {
                self._write(chr);
            }
        }
    }

    pub unsafe fn write_str(&mut self, string: &str) {
        let str_ptr = string.as_ptr();
        for i in 0..string.len() {
            self.write(*str_ptr.add(i))
        }
    }

    fn next_line(&mut self) {
        self.y += 1;
        if self.y >= self.h {
            self.y = 0;
        }
    }

    fn next_char(&mut self) {
        self.x += 1;
        if self.x >= self.w {
            self.next_line();
            self.x = 0;
        }
    }
}

impl Write for VGA {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            self.write_str(s);
        }
        Ok(())
    }
}

pub fn vga_print(args: Arguments<'_>) {
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

pub fn vga_println(args: Arguments<'_>) {
    vga_print(args);
    vga_print(format_args!("\r\n"));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::vga_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::vga::vga_println(format_args!($($arg)*)));
}

unsafe impl Send for VGA {} // fuck you rust

static VGA_WRITER: Mutex<VGA> = Mutex::new(VGA::new());

