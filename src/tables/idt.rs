use crate::drv::io::ioport::inb;
use crate::tables::acpi::LAPIC_ADDR;
use crate::util::panic::_hcf;
use crate::{dprintln, kprintln};
use core::mem::size_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IDT32Entry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attrs: u8,
    isr_high: u16,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct IDT32 {
    limit: u16,
    base: u32,
}

unsafe extern "C" {
    static ISRS: [*const (); 256];
}

#[repr(align(8))]
struct AlignedIDT([IDT32Entry; 256]); // fuck you rust

static mut IDT: AlignedIDT = AlignedIDT(
    [IDT32Entry {
        isr_low: 0,
        isr_high: 0,
        kernel_cs: 0,
        reserved: 0,
        attrs: 0,
    }; 256],
);

static mut IDTR: IDT32 = IDT32 { base: 0, limit: 0 };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn interrupt_handler(interrupt_number: u32, error_code: u32) {
    if interrupt_number < 32 {
        kprintln!(
            "Exception {} (error code {}), halting",
            interrupt_number,
            error_code
        );
        _hcf();
    }

    if interrupt_number == 33 {
        dprintln!("kb");
        inb(0x60);
        *((LAPIC_ADDR.lock().expect("IRQ sent when LAPIC addr == None") + 0xb0) as *mut u32) = 0;
        return;
    }

    kprintln!(
        "{}:{}: unhandled isr {}",
        file!(),
        line!(),
        interrupt_number
    );
}

pub unsafe fn initialize_idt() {
    IDTR.base = &raw const IDT as *const _ as u32;
    IDTR.limit = ((size_of::<IDT32Entry>() as u16) * 256) - 1;

    for (i, _) in ISRS.iter().enumerate() {
        IDT.0[i].isr_low = ((ISRS[i].addr() as u32) & 0xFFFF) as u16;
        IDT.0[i].kernel_cs = 0x08;
        IDT.0[i].attrs = 0x8E;
        IDT.0[i].isr_high = ((ISRS[i].addr() as u32) >> 16) as u16;
        IDT.0[i].reserved = 0;
    }

    dprintln!("Created IDT");
    load_idt(&raw const IDTR);
    dprintln!("Loaded IDT");
}

unsafe extern "C" {
    pub unsafe fn load_idt(idt: *const IDT32);
    pub unsafe fn store_idt(dest: *mut IDT32);
}
