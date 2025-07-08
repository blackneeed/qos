use hashbrown::HashMap;
use spin::Mutex;

use crate::arch::core::Core;
use crate::drv::io::mm::ioapic::IOAPIC;
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
#[derive(Debug, Clone)]
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

type IRQHashMap = HashMap<u8, unsafe fn()>; // clippy keeps complaining about some complex type bullshit

static IDTR: Mutex<Option<IDT32>> = Mutex::new(None);
static IRQS: Mutex<Option<IRQHashMap>> = Mutex::new(None);

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

    if let Some(handler) = IRQS
        .lock()
        .as_ref()
        .unwrap()
        .get(&((interrupt_number - 32) as u8))
    {
        handler();
        core::ptr::write_volatile(
            (LAPIC_ADDR.lock().expect("IRQ sent when LAPIC addr == None") + 0xb0) as *mut u32,
            0,
        );
    }
}

pub unsafe fn initialize_idt() {
    *IRQS.lock() = Some(HashMap::new());
    *IDTR.lock() = Some(IDT32 {
        base: &raw const IDT as u32,
        limit: ((size_of::<IDT32Entry>() as u16) * 256) - 1,
    });

    for (i, _) in ISRS.iter().enumerate() {
        IDT.0[i].isr_low = ((ISRS[i].addr() as u32) & 0xFFFF) as u16;
        IDT.0[i].kernel_cs = 0x08;
        IDT.0[i].attrs = 0x8E;
        IDT.0[i].isr_high = ((ISRS[i].addr() as u32) >> 16) as u16;
        IDT.0[i].reserved = 0;
    }

    dprintln!("Created IDT");
}

pub unsafe fn load_idt() {
    let idtr = IDTR
        .lock()
        .clone()
        .expect("load_idt called before initialization of IDT (before initialize_idt)")
        .clone();

    _lidt(&raw const idtr);
    dprintln!("Loaded IDT");
}

pub unsafe fn register_irq(irq: u8, func: unsafe fn()) {
    let mut lock = IRQS.lock();
    let val = lock.as_mut().unwrap();
    val.insert(irq, func);

    IOAPIC::redirect_irq(irq, irq as u32 + 32, Core::this().apic_id as u32, false)
        .expect("could not redirect irq (in redirect_irq)");
}

unsafe extern "C" {
    unsafe fn _lidt(idt: *const IDT32);
}
