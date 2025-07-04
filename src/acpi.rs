use crate::{
    ioport::outb,
    multiboot::{MultibootInfoTag, get_tag},
    println,
};

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
    pub unsafe fn rsdt_pointers(&self) -> &[u32] {
        core::slice::from_raw_parts(
            (self as *const SDT).add(1) as *const u32,
            ((self.length as usize).saturating_sub(size_of::<SDT>())) / 4,
        )
    }

    pub unsafe fn xsdt_pointers(&self) -> &[u64] {
        core::slice::from_raw_parts(
            (self as *const SDT).add(1) as *const u64,
            ((self.length as usize).saturating_sub(size_of::<SDT>())) / 8,
        )
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
        return f
            .debug_struct("GAS")
            .field("address_space", &self.address_space)
            .field("bit_width", &self.bit_width)
            .field("bit_offset", &self.bit_offset)
            .field("access_size", &self.access_size)
            .field("address", &(self.address - 0))
            .finish();
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
        while addr < 0x000FFFFF {
            if core::slice::from_raw_parts::<u8>(addr as *const u8, 8) == b"RSD PTR " {
                return Some(addr as *const RSDP);
            }
            addr += 16;
        }
        println!("{}:{}: could not find RSDP", file!(), line!());
        None
    }
}

pub unsafe fn get_sdt(signature: &[u8; 4]) -> Option<&'static SDT> {
    if let Some(rsdp) = get_rsdp() {
        match (*rsdp).revision {
            0 => Some(
                (*((*rsdp).rsdt_addr as *const SDT))
                    .rsdt_pointers()
                    .iter()
                    .map(|&x| &*(x as *const SDT))
                    .find(|&x| &x.signature == signature)?,
            ),
            _ => {
                let xsdt_addr = (*(rsdp as *const XSDP)).xsdt_addr;
                if xsdt_addr >= u32::MAX as u64 {
                    println!(
                        "{}:{}: XSDT address > 4GB, can't access XSDT",
                        file!(),
                        line!()
                    );
                    return None;
                }

                (*(xsdt_addr as *const SDT))
                    .xsdt_pointers()
                    .iter()
                    .map(|&x| match x > u32::MAX as u64 {
                        false => Some(&*(x as *const SDT)),
                        true => {
                            println!(
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
        println!(
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
}
