use crate::dprintln;
use crate::drv::fb::flanterm::{FlantermContext, flanterm_fb_init, flanterm_write};
use crate::drv::io::mm::fb::Framebuffer;
use core::fmt::{self, Arguments, Write};

use core::ffi::c_void;
use core::ptr::null;

pub struct TTY {
    ft_ctx: *mut FlantermContext,
}

impl TTY {
    pub fn new(fb: Framebuffer) -> TTY {
        let inst = TTY {
            ft_ctx: unsafe {
                flanterm_fb_init(
                    None,
                    None,
                    fb.addr as *mut u32,
                    fb.width as usize,
                    fb.height as usize,
                    fb.pitch as usize,
                    fb.red_mask_size,
                    fb.red_field_pos,
                    fb.green_mask_size,
                    fb.green_field_pos,
                    fb.blue_mask_size,
                    fb.blue_field_pos,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<u32>() as *mut u32,
                    null::<c_void>() as *mut c_void,
                    0,
                    0,
                    1,
                    0,
                    0,
                    0,
                )
            },
        };

        dprintln!("Initialized TTY");
        inst
    }

    pub fn write_str(&mut self, string: &str) {
        unsafe {
            flanterm_write(self.ft_ctx, string.as_ptr(), string.len());
        }
    }
}

impl Write for TTY {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

pub fn tty_print(args: Arguments<'_>) {
    if let Some(writer) = TTY_WRITER.lock().as_mut() {
        writer.write_fmt(args).unwrap();
    }
}

pub fn tty_println(args: Arguments<'_>) {
    tty_print(args);
    tty_print(format_args!("\r\n"));
}

unsafe impl Send for TTY {}

static TTY_WRITER: spin::Mutex<Option<TTY>> = spin::Mutex::new(None);

pub fn initialized() -> bool {
    return TTY_WRITER.lock().is_some();
}

pub fn init(fb: Framebuffer) {
    *TTY_WRITER.lock() = Some(TTY::new(fb));
}
