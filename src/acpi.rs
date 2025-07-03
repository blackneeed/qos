use crate::multiboot::{MultibootAcpiOldTag, MultibootInfo, get_tag};

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
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct FADT {
    pub firmware_ctrl: u32,
    pub dsdt_addr: u32,
    pub interrupt_model: u8,
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
    pub reserved: u16,
    pub reserved2: u8,
    pub flags: u32,
    pub reset_register: GAS,
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

pub unsafe fn get_acpi_tag(mb2_info: *const MultibootInfo) -> Option<*const MultibootAcpiOldTag> {
    get_tag(mb2_info, 14).map(|x| x as *const MultibootAcpiOldTag)
}

pub unsafe fn get_sdts(
    acpi_tag: *const MultibootAcpiOldTag,
) -> Option<impl Iterator<Item = &'static SDT>> {
    match (*acpi_tag).revision {
        0 => Some(
            (*((*acpi_tag).rsdt_addr as *const SDT))
                .rsdt_pointers()
                .iter()
                .map(|&x| &*(x as *const SDT)),
        ),
        _ => None,
    }
}

pub unsafe fn get_sdt(
    acpi_tag: *const MultibootAcpiOldTag,
    signature: &[u8; 4],
) -> Option<&'static SDT> {
    match (*acpi_tag).revision {
        0 => Some(
            (*((*acpi_tag).rsdt_addr as *const SDT))
                .rsdt_pointers()
                .iter()
                .map(|&x| &*(x as *const SDT))
                .find(|&x| &x.signature == signature)?,
        ),
        _ => None,
    }
}
