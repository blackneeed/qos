use core::mem::size_of;
use crate::panic::_hcf;
use crate::println;
use core::marker::{Copy};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IDT32Entry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attrs: u8,
    isr_high: u16
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct IDT32 {
    limit: u16,
    base: u32
}

unsafe extern "C" {
    static ISRS: [*const (); 256];
}

#[repr(align(8))]
struct AlignedIDT([IDT32Entry; 256]); // fuck you rust

static mut IDT: AlignedIDT = AlignedIDT([IDT32Entry { isr_low: 0, isr_high: 0, kernel_cs: 0, reserved: 0, attrs: 0 }; 256]);

static mut IDTR: IDT32 = IDT32 { base: 0, limit: 0 };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn interrupt_handler(interrupt_number: u32, _error_code: u32) {
    println!("ISR {:03}, halting", interrupt_number);
    _hcf();
}

pub unsafe fn init_idt()
{
    IDTR.base = &raw const IDT as *const _ as u32;
    IDTR.limit = ((size_of::<IDT32Entry>() as u16) * 256) - 1;

    for vec in 0..256 {
        IDT.0[vec].isr_low = ((ISRS[vec].addr() as u32) & 0xFFFF) as u16;
        IDT.0[vec].kernel_cs = 0x08;
        IDT.0[vec].attrs = 0x8E;
        IDT.0[vec].isr_high = ((ISRS[vec].addr() as u32) >> 16) as u16;
        IDT.0[vec].reserved = 0;
    }

    load_idt(&raw const IDTR);
    println!("Loaded IDTR {:#016X}!", (&raw const IDTR) as u32);
}

unsafe extern "C" {
    pub unsafe fn load_idt(idt: *const IDT32);
    pub unsafe fn store_idt(dest: *mut IDT32);
}