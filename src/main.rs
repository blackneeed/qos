// Clippy
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::new_without_default)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
// Features
#![feature(fn_traits)]
#![feature(ascii_char)]
#![feature(custom_test_frameworks)]
// Misc
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::util::test::runner)]
#![no_std]
#![no_main]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod drv;
pub mod mem;
pub mod tables;
pub mod util;

use core::alloc::Layout;

use crate::boot::multiboot::MultibootInfo;
use crate::drv::fb::tty::tty_init;
use crate::drv::io::mm::fb::Framebuffer;
use crate::drv::io::mm::ioapic::ioapic_init;
use crate::drv::io::mm::lapic::lapic_init;
use crate::drv::io::pci::pci_init;
use crate::drv::io::pic::mask_all as pic_mask_all;
use crate::drv::io::ps2::ps2_init;
use crate::drv::io::rtl8139::rtl8139_init;
use crate::mem::allocator::initialize_allocator;
use crate::mem::pmm::get_biggest_usable_pool_multiboot;
use crate::tables::acpi::acpi_init;
use crate::tables::idt::{initialize_idt, load_idt};
use crate::util::misc::{read_volatile_unaligned, write_volatile_unaligned};
use crate::util::panic::infhlt;
use crate::util::sleep::sleep_init;
use alloc::alloc::alloc;
use conquer_once::spin::OnceCell;
use spin::Mutex;

static MULTIBOOT_INFO: Mutex<OnceCell<&'static MultibootInfo>> = Mutex::new(OnceCell::uninit());

pub fn get_multiboot_info() -> &'static MultibootInfo {
    MULTIBOOT_INFO.lock().get().unwrap()
}

fn init_multiboot_info(mb2_info: &'static MultibootInfo) {
    MULTIBOOT_INFO.lock().try_init_once(|| mb2_info).unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    init_multiboot_info(&*mb2_info);
    tty_init(Framebuffer::from_multiboot().expect("framebuffer not available"));
    initialize_allocator(get_biggest_usable_pool_multiboot().expect("no usable memory pools"));
    pic_mask_all();
    initialize_idt();
    load_idt();
    pci_init();
    acpi_init();
    sleep_init();
    lapic_init();
    ioapic_init();
    rtl8139_init();
    ps2_init();
    #[cfg(test)]
    test_main();

    let allocated = alloc(Layout::from_size_align(8, 4).unwrap());
    kprintln!("Allocated 8 bytes at {:x}", allocated as usize);
    kprintln!(
        "Reading u32 at {:x} (not aligned to 4) with read_volatile_unaligned: {:x}",
        allocated.add(2) as usize,
        read_volatile_unaligned(allocated.add(2) as *mut u32)
    );

    kprintln!(
        "Writing u32 {:x} at {:x} (not aligned to 4) with write_volatile_unaligned",
        123u32,
        allocated.add(2) as usize
    );

    write_volatile_unaligned(allocated.add(2) as *mut u32, 123u32);

    kprintln!(
        "Reading u32 at {:x} (not aligned to 4) with read_volatile_unaligned: {:x}",
        allocated.add(2) as usize,
        read_volatile_unaligned(allocated.add(2) as *mut u32)
    );

    infhlt();
}
