#[unsafe(link_section = ".multiboot")]
#[used]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0x2,
    checksum: 0u32.wrapping_sub(0x1BADB002 + 0x2),
};

#[repr(C)]
#[repr(align(8))]
#[derive(Debug)]
pub struct MultibootHeader {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u8; 12], // we aint gonna be using this for now
    pub __pad: u32, // uhh? 
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub color_info: [u8; 5] // we aint gonna be using this for now
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct MultibootMemoryMapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub type_: u32
}