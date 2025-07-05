use crate::boot::multiboot::{MultibootInfoTag, get_tag};
use crate::drv::io::ioport::outb;
use crate::kprintln;

#[repr(C, packed)]
#[derive(Debug)]
pub struct RSDP {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct XSDP {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,

    pub length: u32,
    pub xsdt_addr: u64,
    pub checksum2: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct SDT {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl SDT {
    pub unsafe fn rsdt_pointers(&self) -> impl Iterator<Item = u32> {
        core::slice::from_raw_parts(
            (self as *const SDT).add(1) as *const u8,
            (self.length as usize).saturating_sub(size_of::<SDT>()),
        )
        .chunks_exact(4)
        .map(|x| u32::from_le_bytes(x.try_into().unwrap()))
    }

    pub unsafe fn xsdt_pointers(&self) -> impl Iterator<Item = u64> {
        core::slice::from_raw_parts(
            (self as *const SDT).add(1) as *const u8,
            (self.length as usize).saturating_sub(size_of::<SDT>()),
        )
        .chunks_exact(8)
        .map(|x| u64::from_le_bytes(x.try_into().unwrap()))
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct FADT {
    pub frmw_ctrl: u32,
    pub dsdt_addr: u32,
    pub reserved: u8,
    pub preferred_power_mgmt_prof: u8,
    pub sci_interrupt: u16,
    pub smm_interrupt_command_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4bios_request: u8,
    pub pstate_control: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_ctrl_block: u32,
    pub pm1b_ctrl_block: u32,
    pub pm2_ctrl_block: u32,
    pub pm_timer_block: u32,
    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm1_ctrl_length: u8,
    pub pm2_ctrl_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_length: u8,
    pub gpe1_length: u8,
    pub cstate_control: u8,
    pub worst_c2_latency: u16,
    pub worst_c3_latency: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_off: u8,
    pub duty_width: u8,
    pub day_alarm: u8,
    pub month_alarm: u8,
    pub century: u8,
    pub boot_arch_flags: u16,
    pub reserved3: u8,
    pub flags: u32,
    pub reset_register: GAS,
    pub reset_value: u8,
    pub reserved4: u16,
    pub reserved5: u8,
    pub x_frmw_ctrl: GAS,
    pub x_dsdt: GAS,
    pub x_pm1a_event_block: GAS,
    pub x_pm1b_event_block: GAS,
    pub x_pm1a_ctrl_block: GAS,
    pub x_pm1b_ctrl_block: GAS,
    pub x_pm2_ctrl_block: GAS,
    pub x_pm_timer_block: GAS,
    pub x_gpe0_block: GAS,
    pub x_gpe1_block: GAS,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GAS {
    pub address_space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

impl core::fmt::Debug for GAS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GAS")
            .field("address_space", &self.address_space)
            .field("bit_width", &self.bit_width)
            .field("bit_offset", &self.bit_offset)
            .field("access_size", &self.access_size)
            .field("address", &(self.address - 0))
            .finish()
    }
}

pub unsafe fn get_rsdp() -> Option<*const RSDP> {
    if let Some(rsdp) = get_tag(15)
        .map(|x| (x as *const u8).add(core::mem::size_of::<MultibootInfoTag>()) as *const RSDP)
    {
        Some(rsdp)
    } else if let Some(rsdp) = get_tag(14)
        .map(|x| (x as *const u8).add(core::mem::size_of::<MultibootInfoTag>()) as *const RSDP)
    {
        Some(rsdp)
    } else {
        let mut addr = 0x000E0000u32;
        'outer: while addr < 0x000FFFFF {
            let valid = b"RSD PTR ";

            for (i, &v) in valid.iter().enumerate() {
                if *((addr + i as u32) as *const u8) != v {
                    addr += 16;
                    continue 'outer;
                }
            }

            return Some(addr as *const RSDP);
        }
        panic!("{}:{}: could not find RSDP", file!(), line!());
    }
}

pub unsafe fn get_sdt(signature: &[u8; 4]) -> Option<&'static SDT> {
    if let Some(rsdp) = get_rsdp() {
        match (*rsdp).revision {
            0 => Some(
                (*((*rsdp).rsdt_addr as *const SDT))
                    .rsdt_pointers()
                    .map(|x| &*(x as *const SDT))
                    .find(|&x| &x.signature == signature)?,
            ),
            _ => {
                let xsdt_addr = (*(rsdp as *const XSDP)).xsdt_addr;
                if xsdt_addr >= u32::MAX as u64 {
                    kprintln!(
                        "{}:{}: XSDT address > 4GB, can't access XSDT",
                        file!(),
                        line!()
                    );
                    return None;
                }

                (*(xsdt_addr as *const SDT))
                    .xsdt_pointers()
                    .map(|x| match x > u32::MAX as u64 {
                        false => Some(&*(x as *const SDT)),
                        true => {
                            kprintln!(
                                "{}:{}: found matching SDT but address > 4GB, skipping",
                                file!(),
                                line!()
                            );
                            None
                        }
                    })
                    .find(|&x| x.is_some() && &x.unwrap().signature == signature)?
            }
        }
    } else {
        kprintln!(
            "{}:{}: couldn't find SDT '{}'",
            file!(),
            line!(),
            core::str::from_utf8_unchecked(signature)
        );
        None
    }
}

pub unsafe fn get_fadt() -> Option<&'static FADT> {
    if let Some(sdt) = get_sdt(b"FACP") {
        Some(&*(((&raw const *sdt) as *const u8).add(core::mem::size_of::<SDT>()) as *const FADT))
    } else {
        None
    }
}

pub unsafe fn acpi_init() {
    if let Some(fadt) = get_fadt()
        && (fadt.smm_interrupt_command_port != 0 || fadt.acpi_enable != 0)
    {
        outb(
            (fadt.smm_interrupt_command_port & 0xFFFF) as u16,
            fadt.acpi_enable,
        );
    }

    if let Some(madt) = get_sdt(b"APIC") {
        kprintln!(
            "Local APIC address: {:#08X}",
            core::ptr::read_unaligned(
                ((&raw const *madt) as *const u8)
                    .add(core::mem::size_of::<SDT>())
                    .add(0) as *const u32
            ),
        );

        kprintln!(
            "Legacy 8259 PICs{} installed",
            match ((core::ptr::read_unaligned(
                ((&raw const *madt) as *const u8)
                    .add(core::mem::size_of::<SDT>())
                    .add(4) as *const u32
            )) & 1)
                != 0
            {
                true => "",
                false => "not ",
            }
        );

        let mut address = ((&raw const *madt) as *const u8)
            .add(core::mem::size_of::<SDT>())
            .add(8);

        let mut length = core::mem::size_of::<SDT>() as u32 + 8;

        while length < madt.length {
            let entry_type = *address;
            let entry_length = *address.add(1);
            let entry_address = address.add(2);

            match entry_type {
                0 => {
                    kprintln!("Processor LAPIC");
                    kprintln!("\tACPI Processor ID: {}", *entry_address);
                    kprintln!("\tAPIC ID: {}", *entry_address.add(1));
                    kprintln!(
                        "\tprocessor can{} be enabled",
                        if (((*entry_address.add(2)) & 1) | ((*entry_address.add(2)) & 2)) != 0 {
                            ""
                        } else {
                            "'t"
                        }
                    );
                }
                1 => {
                    kprintln!("I/O APIC");
                    kprintln!("\tID: {}", *entry_address);
                    kprintln!(
                        "\tAddress: {}",
                        core::ptr::read_unaligned(entry_address.add(2) as *const u32)
                    );
                    kprintln!(
                        "\tGSI Base: {}",
                        core::ptr::read_unaligned(entry_address.add(6) as *const u32)
                    );
                }
                2 => {
                    kprintln!("I/O APIC Interrupt Source Override");
                    kprintln!("\tBus source: {}", *entry_address);
                    kprintln!("\tIRQ source: {}", *entry_address.add(1));
                    kprintln!(
                        "\tGSI: {}",
                        core::ptr::read_unaligned(entry_address.add(2) as *const u32)
                    );
                    kprintln!(
                        "\tFlags: {}",
                        core::ptr::read_unaligned(entry_address.add(6) as *const u16)
                    );
                }
                3 => {
                    kprintln!("I/O APIC Non-maskable interrupt source");
                    kprintln!("\tNMI Source: {}", *entry_address);
                    kprintln!(
                        "\tFlags: {}",
                        core::ptr::read_unaligned(entry_address.add(2) as *const u16)
                    );
                    kprintln!(
                        "\tGSI: {}",
                        core::ptr::read_unaligned(entry_address.add(4) as *const u32)
                    );
                }
                4 => {
                    kprintln!("LAPIC Non-maskable interrupts");
                    kprintln!(
                        "\tACPI Processor ID: {}{}",
                        *entry_address,
                        if *entry_address == 0xFF { " (all)" } else { "" }
                    );
                    kprintln!(
                        "\tFlags: {}",
                        core::ptr::read_unaligned(entry_address.add(1) as *const u16)
                    );
                    kprintln!("\tLINT{}", *entry_address.add(3));
                }
                5 => {
                    kprintln!(
                        "{}:{}: found LAPIC address override entry in MADT, skipping!",
                        file!(),
                        line!()
                    )
                }
                9 => {
                    kprintln!(
                        "{}:{}: found x2APIC entry in MADT, skipping!",
                        file!(),
                        line!()
                    );
                }
                _ => {
                    kprintln!(
                        "{}:{}: found unknown entry type {} in MADT, skipping!",
                        file!(),
                        line!(),
                        entry_type
                    );
                }
            }

            address = address.add(entry_length as usize);
            length += entry_length as u32;
        }
    }
}
