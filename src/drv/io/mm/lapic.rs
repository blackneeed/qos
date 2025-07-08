use crate::arch::msr::wrmsr;
use crate::initialized;
use crate::tables::acpi::LAPIC_ADDR;

pub unsafe fn lapic_init() {
    let addr = LAPIC_ADDR
        .lock()
        .expect("init_lapic called before initialization of MADT");

    core::ptr::write_volatile(
        (addr + 0xF0) as *mut u32,
        core::ptr::read_volatile((addr + 0xF0) as *const u32) | 0x100,
    );

    wrmsr(0x1B, (1 << 8) | (1 << 11) | (addr as u64));

    initialized!("LAPIC");
}
