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
#[derive(Debug, Copy, Clone)]
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

