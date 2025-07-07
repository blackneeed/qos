use crate::boot::multiboot::{MultibootFramebufferTag, get_tag};
use crate::mem::pmm::memset32;
use crate::{dprintln, kprintln};

use alloc::alloc::alloc;
use core::alloc::Layout;

pub unsafe fn get_framebuffer_tag() -> Option<*const MultibootFramebufferTag> {
    get_tag(8).map(|x| x as *const MultibootFramebufferTag)
}

#[derive(Debug)]
pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub addr: *mut u8,
    pub red_field_pos: u8,
    pub red_mask_size: u8,
    pub green_field_pos: u8,
    pub green_mask_size: u8,
    pub blue_field_pos: u8,
    pub blue_mask_size: u8,
    pub bpp: u8,
    pub bpp_b: u8,
    pub double_fb: *mut u8,
    pub mem_len: u32,
}

impl Framebuffer {
    pub unsafe fn new(
        width: u32,
        height: u32,
        pitch: u32,
        addr: *mut u8,
        red_field_pos: u8,
        red_mask_size: u8,
        green_field_pos: u8,
        green_mask_size: u8,
        blue_field_pos: u8,
        blue_mask_size: u8,
        bpp: u8,
    ) -> Option<Framebuffer> {
        if bpp != 32 {
            return None;
        }

        if let Ok(layout) = Layout::from_size_align(height as usize * pitch as usize, 4) {
            let double_fb = alloc(layout);
            if double_fb.is_null() {
                kprintln!(
                    "{}:{}: could not allocate double framebuffer",
                    file!(),
                    line!()
                );
                return None;
            }

            core::ptr::write_bytes(double_fb, 0, height as usize * pitch as usize);

            dprintln!("Initialized framebuffer");
            Some(Framebuffer {
                width,
                height,
                pitch,
                addr,
                red_field_pos,
                red_mask_size,
                green_field_pos,
                green_mask_size,
                blue_field_pos,
                blue_mask_size,
                bpp: 32,
                bpp_b: 4,
                double_fb,
                mem_len: height * pitch,
            })
        } else {
            kprintln!(
                "{}:{}: could not create layout for double framebuffer",
                file!(),
                line!()
            );
            None
        }
    }

    pub unsafe fn from_multiboot() -> Option<Framebuffer> {
        if let Some(tag) = get_framebuffer_tag() {
            Framebuffer::new(
                (*tag).width,
                (*tag).height,
                (*tag).pitch,
                (*tag).addr as *mut u8,
                (*tag).red_field_pos,
                (*tag).red_mask_size,
                (*tag).green_field_pos,
                (*tag).green_mask_size,
                (*tag).blue_field_pos,
                (*tag).blue_mask_size,
                (*tag).bpp,
            )
        } else {
            None
        }
    }

    #[inline(always)]
    fn conv_color(&self, value: u8, n: u8) -> u8 {
        if n == 8 {
            return value;
        };
        ((value as u32 * ((1u32 << n as u32) - 1u32) + 128u32) >> 8) as u8
    }

    #[inline(always)]
    fn color_packed(&self, col: u32) -> u32 {
        let r = (self.conv_color(((col >> 16) & 0xFF) as u8, self.red_mask_size) as u32)
            << self.red_field_pos;
        let g = (self.conv_color(((col >> 8) & 0xFF) as u8, self.green_mask_size) as u32)
            << self.green_field_pos;
        let b = (self.conv_color((col & 0xFF) as u8, self.blue_mask_size) as u32)
            << self.blue_field_pos;

        r | g | b
    }

    #[inline(always)]
    unsafe fn get_pixel_ptr(&self, x: u32, y: u32) -> *mut u32 {
        (self
            .double_fb
            .add((y * self.pitch + x * self.bpp_b as u32) as usize)) as *mut u32
    }

    #[inline(always)]
    pub unsafe fn put_pixel(&self, col: u32, x: u32, y: u32) // 0xAARRGGBB (alpha unhandled)
    {
        *self.get_pixel_ptr(x, y) = self.color_packed(col);
    }

    pub unsafe fn draw_line(&mut self, col: u32, w: u32, x: u32, y: u32) {
        memset32(self.get_pixel_ptr(x, y), self.color_packed(col), w);
    }

    pub unsafe fn swap(&self) {
        core::ptr::copy_nonoverlapping(self.double_fb, self.addr, self.mem_len as usize);
    }
}
