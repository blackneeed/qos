use crate::dprintln;
use crate::tables::acpi::{IOAPIC_ISOS, IOAPICS, LAPIC_ADDR, MADTIOAPIC};
use spin::Mutex;

const IOREGSEL_OFF: u32 = 0x00;
const IOREGWIN_OFF: u32 = 0x10;
const IOAPICID_REG: u32 = 0x00;
const IOAPICVER_REG: u32 = 0x01;

pub struct IOAPIC {
    pub madt: MADTIOAPIC,
    pub version: u8,
    pub redir_entries: u8,
}

static LAPIC_ENABLED: Mutex<bool> = Mutex::new(false);

impl IOAPIC {
    pub unsafe fn new(madt: MADTIOAPIC) -> Result<IOAPIC, &'static str> {
        *((madt.address as *mut u8).add(IOREGSEL_OFF as usize) as *mut u32) = IOAPICID_REG;
        if ((*((madt.address as *mut u8).add(IOREGWIN_OFF as usize) as *mut u32) >> 24) & 0xFF)
            as u8
            != madt.id
        {
            return Err("APIC ID Mismatch");
        }

        *((madt.address as *mut u8).add(IOREGSEL_OFF as usize) as *mut u32) = IOAPICVER_REG;
        let version = ((*((madt.address as *mut u8).add(IOREGWIN_OFF as usize) as *mut u32) >> 24)
            & 0xFF) as u8;

        *((madt.address as *mut u8).add(IOREGSEL_OFF as usize) as *mut u32) = IOAPICVER_REG;

        let entry_count =
            (*((madt.address as *mut u8).add(IOREGWIN_OFF as usize) as *mut u32) >> 16) as u8 + 1;

        dprintln!("{}-{entry_count}", madt.gsi_base);

        dprintln!("Initialized I/O APIC at {:#08X}", madt.address);

        if !(*LAPIC_ENABLED.lock()) {
            if let Some(addr) = *LAPIC_ADDR.lock() {
                let old = *((addr + 0xF0) as *const u32);
                *((addr + 0xF0) as *mut u32) = old | 0x100;
                *LAPIC_ENABLED.lock() = true;
            } else {
                return Err("LAPIC address not available");
            }
        }

        Ok(IOAPIC {
            madt,
            version,
            redir_entries: entry_count,
        })
    }

    pub unsafe fn ioapic_for_gsi(gsi: u32) -> Option<IOAPIC> {
        let lock = IOAPICS.lock();
        for &madt_ioapic in &*lock {
            if let Ok(ioapic) = IOAPIC::new(madt_ioapic)
                && gsi >= ioapic.madt.gsi_base
                && gsi <= ioapic.madt.gsi_base + ioapic.redir_entries as u32
            {
                return Some(ioapic);
            }
        }
        None
    }

    pub unsafe fn gsi_for_irq(irq: u8) -> u32 {
        for iso in &*IOAPIC_ISOS.lock() {
            if iso.irq == irq {
                return iso.gsi;
            }
        }

        irq as u32
    }

    pub unsafe fn flags_for_irq(irq: u8) -> u16 {
        for iso in &*IOAPIC_ISOS.lock() {
            if iso.irq == irq {
                return iso.flags;
            }
        }

        0
    }

    // flags is from ISO (can be 0!)
    pub unsafe fn redirect_gsi(&self, vec: u32, lapic: u32, gsi: u32, flags: u16) {
        if gsi < self.madt.gsi_base || gsi > self.madt.gsi_base + self.redir_entries as u32 {
            dprintln!(
                "attempt to redirect gsi was made on a ioapic that doesnt handle this gsi ({} -> {} on {}-{} gsi handling ioapic)",
                gsi,
                vec,
                self.madt.gsi_base,
                self.madt.gsi_base + self.redir_entries as u32
            );
            return;
        }

        /*
               struct
               {
                       uint64_t vector       : 8; vec
                       uint64_t delvMode     : 3; 0 (fixed)
                       uint64_t destMode     : 1; 0 (phys)
                       uint64_t delvStatus   : 1; (0 = idle 1 = pending)
                       uint64_t pinPolarity  : 1; 0 = active high 1 = active low
                       uint64_t remoteIRR    : 1; (for level-triggered: 1 = int accepted 0 = eoi received)
                       uint64_t triggerMode  : 1; 0 = edge triggered 1 = level triggered
                       uint64_t mask         : 1; 0 = int on 1 = int off
                       uint64_t reserved     : 39; 0
                       uint64_t destination  : 8; lapic id
                };
        */

        /*  flags bit 1 = polarity, flags bit 3 = trigger mode */
        // 0: Edge, 1: Level. For ISA IRQs assume Edge unless otherwise specified in Interrupt Source Override descriptors of the MADT or in the MP Tables
        // 0: Active high, 1: Active low. For ISA IRQs assume Active High unless otherwise specified in Interrupt Source Override descriptors of the MADT or in the MP Tables.. we dont even nede to set them ez

        let mut val: u64 = vec as u64;
        val |= (((flags & (1 << 1)) >> 1) as u64) << 13;
        val |= (((flags & (1 << 3)) >> 1) as u64) << 15;
        val |= (lapic as u64) << 56;

        dprintln!("GSI: {}, Vector: {}, LAPIC: {}", gsi, vec, lapic);

        // Following there are two 32-bit register for each IRQ. The first IRQ has indexes 0x10 and 0x11, the second 0x12 and 0x13, the third 0x14 and 0x15, and so on. So the Redirection Entry register for IRQ n is 0x10 + n * 2 (+ 1). In the first of the two registers you access to the LOW uint32_t / bits 31:0, and the second for the high uint32_t / 63:32. Each redirection entry is made of the following fields:
        dprintln!(
            "Value: {:#064b}\r\n{:#032b} -> {:#08X}\r\n{:#032b} -> {:#08X}",
            val,
            (val & 0xFFFFFFFF),
            self.madt.address as usize + 0x10 + gsi as usize * 2,
            (val >> 32) as u32,
            self.madt.address as usize + 0x10 + gsi as usize * 2 + 1
        );
        *((self.madt.address as *mut u8).add(0x10 + gsi as usize * 2) as *mut u32) =
            (val & 0xFFFFFFFF) as u32;
        *((self.madt.address as *mut u8).add(0x10 + gsi as usize * 2 + 1) as *mut u32) =
            (val >> 32) as u32;
    }
}
