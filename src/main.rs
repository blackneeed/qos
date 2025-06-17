#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]

use core::panic::PanicInfo;
use core::arch::asm;
use core::fmt::{self, Arguments};
use core::fmt::{Write};
use spin::Mutex;
use core::cmp::min;

#[unsafe(link_section = ".multiboot")]
#[used]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0x2,
    checksum: 0u32.wrapping_sub(0x1BADB002 + 0x2),
};

#[repr(C)]
#[repr(align(8))]
#[derive(Debug)]
pub struct MultibootHeader {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u8; 12], // we aint gonna be using this for now
    pub __pad: u32, // uhh? 
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub color_info: [u8; 5] // we aint gonna be using this for now
}

#[repr(C)]
#[derive(Debug)]
pub struct MultibootMemoryMapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32
}

pub struct VGA {
    vga_ptr: *mut u8,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

pub struct MemoryPool {
    pub start: *mut u8,
    pub length: u32
}

unsafe extern "C" {
    static KERNEL_START: ();
    static KERNEL_END: ();
}

impl VGA {
    pub const fn new() -> VGA
    {
        return VGA { vga_ptr: 0xb8000 as *mut u8, x: 0, y: 0, w: 80, h: 25 };
    }

    unsafe fn get_ptr(&mut self) -> *mut u8
    {
        return self.vga_ptr.add(((self.y * self.w + self.x) * 2) as usize);
    }

    unsafe fn _write(&mut self, chr: u8)
    {
        *self.get_ptr() = chr;
        *self.get_ptr().add(1) = 0x0F;
        self.next_char();
        let abs_pos: u16 = self.y * self.w + self.x;
        outb(0x3D4, 0xE);
        outb(0x3D5, (abs_pos >> 8) as u8);
        outb(0x3D4, 0xF);
        outb(0x3D5, abs_pos as u8);
    }

    pub unsafe fn write(&mut self, chr: u8)
    {
        match chr
        {
            b'\r' => {
                self.x = 0;
            }
            b'\n' => {
                self.next_line();
            }
            b'\t' => {
                for _ in 0..4
                {
                    self._write(b' ');
                }
            }
            _ => {
                self._write(chr);
            }
        }
    }

    pub unsafe fn write_str(&mut self, string: &str)
    {
        let str_ptr = string.as_ptr();
        for i in 0..string.len()
        {
            self.write(*str_ptr.add(i))
        }
    }

    fn next_line(&mut self)
    {
        self.y += 1;
        if self.y >= self.h
        {
            self.y = 0;
        }
    }

    fn next_char(&mut self)
    {
        self.x += 1;
        if self.x >= self.w
        {
            self.next_line();
            self.x = 0;
        }
    }
}

impl Write for VGA {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { self.write_str(s); }
        Ok(())
    }
}

unsafe impl Send for VGA {} // fuck you rust

static VGA_WRITER: Mutex<VGA> = Mutex::new(VGA::new());

pub unsafe fn inb(port: u16) -> u8 {
    let rv: u8;
    asm!("in al, dx", out("al") rv, in("dx") port);
    return rv;
}

pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("al") val, in("dx") port);
}

pub fn vga_print(args: Arguments<'_>)
{
    VGA_WRITER.lock().write_fmt(args).unwrap();
}

pub fn vga_println(args: Arguments<'_>)
{
    vga_print(args);
    vga_print(format_args!("\r\n"));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::vga_println(format_args!($($arg)*)));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    let mut biggest_usable_memory_pool: MemoryPool = MemoryPool { start: &mut 0, length: 0 };
    for entry in core::slice::from_raw_parts((*mb2_info).mmap_addr as *const MultibootMemoryMapEntry, ((*mb2_info).mmap_length / size_of::<MultibootMemoryMapEntry>() as u32) as usize)
    {
        if (*entry).base_addr > u32::MAX as u64
        {
            println!("block {:#016X}-{:#016X}:", (*entry).base_addr, (*entry).base_addr + (*entry).length);
            println!("\tstart address {:#016X} is bigger than 32-bit max ({:#016X}), block is unusable.", (*entry).base_addr, u32::MAX);
            continue;
        } else if (*entry).base_addr + (*entry).length > u32::MAX as u64 {
            println!("block {:#016X}-{:#016X}:", (*entry).base_addr, (*entry).base_addr + (*entry).length);
            println!("\tend address {:#016X} is bigger than 32-bit max ({:#016X}), block needs to be chopped down to {:#016X}-{:#016X} from {:#016X}-{:#016X}.", (*entry).base_addr + (*entry).length, u32::MAX, (*entry).base_addr, u32::MAX, (*entry).base_addr, (*entry).base_addr + (*entry).length);
        }

        let type_str: &str = match (*entry).type_ {
            1 => "Available",
            2 => "Reserved",
            3 => "ACPI",
            4 => "NVS",
            5 => "Defective RAM",
            _ => "Unknown",
        };

        print!("{:#016X}-{:#016X} ({})", (*entry).base_addr as u32, min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32, type_str);
        if (*entry).type_ == 1 && min((*entry).length as u32, u32::MAX - (*entry).base_addr as u32) > biggest_usable_memory_pool.length
        {
            biggest_usable_memory_pool.start = (*entry).base_addr as u32 as *mut u8;
            biggest_usable_memory_pool.length = min((*entry).length as u32, u32::MAX - (*entry).base_addr as u32);
            print!(" (promoted to new biggest usable pool)");
        }
        println!();
    }

    println!("biggest usable memory pool: {:#016X}-{:#016X}", biggest_usable_memory_pool.start.addr() as u32, biggest_usable_memory_pool.start.addr() as u32 + biggest_usable_memory_pool.length);
    let kernel_start: u32 = (&KERNEL_START as *const ()).addr() as u32;
    let kernel_end: u32 = (&KERNEL_END as *const ()).addr() as u32;
    println!("kernel: {:#016X}-{:#016X}", kernel_start, kernel_end);
    if kernel_start >= biggest_usable_memory_pool.start.addr() as u32 && kernel_end <= biggest_usable_memory_pool.start.addr() as u32 + biggest_usable_memory_pool.length
    {
        println!("biggest usable memory pool and kernel overlap");
    } else {
        println!("biggest usable memory pool and kernel do not overlap");
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}