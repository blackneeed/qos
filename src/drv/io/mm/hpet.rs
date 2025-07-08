use crate::initialized;
use crate::tables::acpi::get_hpet;
use core::arch::asm;
use core::time::Duration;

pub struct HPET {
    counter_clock_period: u64,
    address: u32,
}

impl HPET {
    pub unsafe fn new() -> HPET {
        let hpet_acpi = get_hpet().expect("No HPET installed");
        assert_eq!(
            hpet_acpi.base_address.address_space, 0,
            "HPET Address Space != 0"
        );

        let address = hpet_acpi
            .base_address
            .address
            .try_into()
            .expect("HPET Address > 4GB");

        let counter_clock_period = core::ptr::read_volatile(address as *const u64) >> 32;

        /*
        General Configuration Register
        0>ENABLE_CNF	Overall enable.
        0 - main counter is halted, timer interrupts are disabled

        1 - main counter is running, timer interrupts are allowed if enabled
        */

        core::ptr::write_volatile((address as *mut u8).add(0x10) as *mut u64, 0); // disable
        core::ptr::write_volatile((address as *mut u8).add(0xF0) as *mut u64, 0); // clear counter
        core::ptr::write_volatile((address as *mut u8).add(0x10) as *mut u64, 1); // enable

        initialized!("HPET");
        HPET {
            address,
            counter_clock_period,
        }
    }

    pub unsafe fn sleep(&self, dur: Duration) {
        let mic = dur.as_micros();
        let pass = mic * 1_000_000_000 / self.counter_clock_period as u128; // counter clock period is in femtoseconds/tick, so we need to divide mic * femto/micro by the counter clock period
        let addr = (self.address as *const u8).add(0xF0) as *const u64;
        let start = core::ptr::read_volatile(addr);

        while (core::ptr::read_volatile(addr) as u128) < pass + start as u128 {
            asm!("pause");
        }
    }
}
