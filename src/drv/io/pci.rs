use crate::drv::io::ioport::{inl, outl};
use crate::initialized;
use alloc::vec::Vec;
use spin::Mutex;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Debug)]
pub struct PCIDevice {
    pub device: u16,
    pub vendor: u16,
    pub status: u16,
    pub command: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub bist: u8,
    pub header_type: u8,
    pub latency_timer: u8,
    pub cache_line_size: u8,
}

pub static PCI_DEVICES: Mutex<Vec<PCIDevice>> = Mutex::new(Vec::new());

unsafe fn pci_config_read_dword(bus: u8, slot: u8, func: u8, off: u8) -> u32 {
    outl(
        CONFIG_ADDRESS,
        ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((off as u32) & 0xFC)
            | 0x80000000u32,
    );

    inl(CONFIG_DATA)
}

pub unsafe fn pci_init() {
    let mut lock = PCI_DEVICES.lock();

    for bus in 0..=255 {
        for slot in 0..32 {
            for func in 0..=7 {
                let dword0 = pci_config_read_dword(bus, slot, func, 0x0);
                let vendor = (dword0 & 0xFFFF) as u16;
                let device = ((dword0 >> 16) & 0xFFFF) as u16;

                if vendor == 0xFFFF {
                    continue;
                }

                let dword4 = pci_config_read_dword(bus, slot, func, 0x4);
                let command = (dword4 & 0xFFFF) as u16;
                let status = ((dword4 >> 16) & 0xFFFF) as u16;

                let dword8 = pci_config_read_dword(bus, slot, func, 0x8);
                let revision = (dword8 & 0xFF) as u8;
                let prog_if = ((dword8 >> 8) & 0xFF) as u8;
                let subclass = ((dword8 >> 16) & 0xFF) as u8;
                let class = ((dword8 >> 24) & 0xFF) as u8;

                let dword12 = pci_config_read_dword(bus, slot, func, 0xC);
                let cache_line_size = (dword12 & 0xFF) as u8;
                let latency_timer = ((dword12 >> 8) & 0xFF) as u8;
                let header_type = ((dword12 >> 16) & 0xFF) as u8;
                let bist = ((dword12 >> 24) & 0xFF) as u8;

                lock.push(PCIDevice {
                    vendor,
                    device,
                    command,
                    status,
                    revision,
                    prog_if,
                    subclass,
                    class,
                    cache_line_size,
                    latency_timer,
                    header_type,
                    bist,
                });
            }
        }
    }

    initialized!("PCI");
}
