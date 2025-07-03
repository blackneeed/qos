use crate::println;

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootInfo {
    pub total_size: u32,
    pub reserved: u32,
    pub tags: (),
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootInfoTag {
    pub type_: u32,
    pub size: u32,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootMemoryMapTag {
    pub type_: u32,
    pub size: u32,
    pub entry_size: u32,
    pub entry_version: u32,
    pub entries: (),
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootMemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32,
    pub zero: u32,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootFramebufferTag {
    pub type_: u32,
    pub size: u32,
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub fb_type: u8,
    pub reserved: u16,
    pub red_field_pos: u8,
    pub red_mask_size: u8,
    pub green_field_pos: u8,
    pub green_mask_size: u8,
    pub blue_field_pos: u8,
    pub blue_mask_size: u8,
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootAcpiOldTag {
    pub type_: u32,
    pub size: u32,
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,
}

pub unsafe fn get_tag(
    mb2_info: *const MultibootInfo,
    type_: u32,
) -> Option<*const MultibootInfoTag> {
    if (*mb2_info).total_size as usize == core::mem::size_of::<MultibootInfo>() {
        return None;
    }

    let mut tag_ptr = (&raw const (*mb2_info).tags) as *const MultibootInfoTag;
    loop {
        if (*tag_ptr).type_ == 0 {
            return None;
        }

        if (*tag_ptr).type_ == type_ {
            println!("g");
            return Some(tag_ptr);
        }

        tag_ptr = (tag_ptr as *const u8).add(((*tag_ptr).size as usize + 7) & !7)
            as *const MultibootInfoTag;
    }
}
