use crate::get_multiboot_info;
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

pub unsafe fn get_tag(type_: u32) -> Option<*const MultibootInfoTag> {
    let mb2_info = get_multiboot_info();
    if (*mb2_info).total_size as usize == core::mem::size_of::<MultibootInfo>() {
        return None;
    }

    let mut tag_ptr = (&raw const (*mb2_info).tags) as *const MultibootInfoTag;
    loop {
        if (*tag_ptr).type_ == 0 {
            println!("{}:{}: tag of type {} not present", file!(), line!(), type_);
            return None;
        }

        if (*tag_ptr).type_ == type_ {
            return Some(tag_ptr);
        }

        tag_ptr = (tag_ptr as *const u8).add(((*tag_ptr).size as usize + 7) & !7)
            as *const MultibootInfoTag;
    }
}
