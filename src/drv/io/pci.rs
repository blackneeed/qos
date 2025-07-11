use crate::drv::io::ioport::{inl, outl};
use crate::initialized;
use alloc::vec::Vec;
use spin::Mutex;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Debug, Clone)]
pub struct PCIDevice {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
    pub device: u16,
    pub vendor: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub bist: u8,
    pub header_type: u8,
    pub latency_timer: u8,
    pub cache_line_size: u8,
}

impl PCIDevice {
    pub fn to_detailed_device<'a>(&'a mut self) -> DetailedPCIDevice<'a> {
        match self.header_type & !(1 << 7) {
            0x0 => DetailedPCIDevice::General(GeneralPCIDevice::fetch(self)),
            0x1 => DetailedPCIDevice::PCItoPCIBridgeDevice(PCItoPCIBridgeDevice::fetch(self)),
            0x2 => {
                DetailedPCIDevice::PCItoCardBusBridgeDevice(PCItoCardBusBridgeDevice::fetch(self))
            }
            _ => DetailedPCIDevice::Unknown(self.clone()),
        }
    }

    pub fn read_status(&self) -> u16 {
        let _dword4 = unsafe { pci_config_read_dword(self.bus, self.slot, self.func, 0x4) };
        (_dword4 >> 8) as u16
    }

    pub fn read_command(&self) -> u16 {
        let _dword4 = unsafe { pci_config_read_dword(self.bus, self.slot, self.func, 0x4) };
        (_dword4 & 0xFFFF) as u16
    }

    pub fn write_status(&self, val: u16) {
        let val = self.read_command() as u32 | ((val as u32) << 16);
        unsafe {
            pci_config_write_dword(self.bus, self.slot, self.func, 0x4, val);
        }
    }

    pub fn write_command(&self, val: u16) {
        let val = ((self.read_status() as u32) << 16) | (val as u32);
        unsafe {
            pci_config_write_dword(self.bus, self.slot, self.func, 0x4, val);
        }
    }
}

#[derive(Debug)]
pub enum DetailedPCIDevice<'a> {
    General(GeneralPCIDevice<'a>),
    PCItoPCIBridgeDevice(PCItoPCIBridgeDevice<'a>),
    PCItoCardBusBridgeDevice(PCItoCardBusBridgeDevice<'a>),
    Unknown(PCIDevice),
}

#[derive(Debug, Clone)]
pub enum PCIBar {
    Memory(PCIMemoryBar),
    IO(PCIIOBar),
    Unknown(u32),
}

impl PCIBar {
    pub fn from_u32(bar: u32) -> PCIBar {
        if PCIMemoryBar::is_mem(bar) {
            PCIBar::Memory(PCIMemoryBar::from_u32(bar))
        } else if PCIIOBar::is_io(bar) {
            PCIBar::IO(PCIIOBar::from_u32(bar))
        } else {
            PCIBar::Unknown(bar)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PCIMemoryBar {
    pub prefetchable: bool,
    pub address: u32,
}

impl PCIMemoryBar {
    pub fn is_mem(bar: u32) -> bool {
        ((bar & 1) == 0) && (((bar << 4) >> 4) != 0)
    }

    pub fn from_u32(bar: u32) -> PCIMemoryBar {
        PCIMemoryBar {
            prefetchable: ((bar >> 3) & 1) != 0,
            address: (bar >> 4) << 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PCIIOBar {
    pub address: u32,
}

impl PCIIOBar {
    pub fn is_io(bar: u32) -> bool {
        (bar & 1) != 0
    }

    pub fn from_u32(bar: u32) -> PCIIOBar {
        PCIIOBar {
            address: (bar >> 2) << 2,
        }
    }
}

#[derive(Debug)]
pub struct GeneralPCIDevice<'a> {
    pub base: &'a mut PCIDevice,
    pub bar0: PCIBar,
    pub bar1: PCIBar,
    pub bar2: PCIBar,
    pub bar3: PCIBar,
    pub bar4: PCIBar,
    pub bar5: PCIBar,
    pub cardbus_cis_ptr: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub expansion_rom_address: u32,
    pub capabilities_ptr: u8,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_latency: u8,
}

impl<'a> GeneralPCIDevice<'a> {
    pub fn fetch(dev: &'a mut PCIDevice) -> Self {
        let (
            dword16,
            dword20,
            dword24,
            dword28,
            dword32,
            dword36,
            dword40,
            dword44,
            dword48,
            dword52,
            _dword56, // this whole shit is reserved
            dword60,
        ) = unsafe {
            (
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x10),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x14),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x18),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x1C),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x20),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x24),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x28),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x2C),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x30),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x34),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x38),
                pci_config_read_dword(dev.bus, dev.slot, dev.func, 0x3C),
            )
        };

        Self {
            base: dev,
            bar0: PCIBar::from_u32(dword16),
            bar1: PCIBar::from_u32(dword20),
            bar2: PCIBar::from_u32(dword24),
            bar3: PCIBar::from_u32(dword28),
            bar4: PCIBar::from_u32(dword32),
            bar5: PCIBar::from_u32(dword36),
            cardbus_cis_ptr: dword40,
            subsystem_vendor_id: (dword44 & 0xFFFF) as u16,
            subsystem_id: (dword44 >> 16) as u16,
            expansion_rom_address: dword48,
            capabilities_ptr: (dword52 & 0xFF) as u8,
            interrupt_line: (dword60 & 0xFF) as u8,
            interrupt_pin: ((dword60 >> 8) & 0xFF) as u8,
            min_grant: ((dword60 >> 16) & 0xFF) as u8,
            max_latency: ((dword60 >> 24) & 0xFF) as u8,
        }
    }
}

#[derive(Debug)]
pub struct PCItoPCIBridgeDevice<'a> {
    pub base: &'a mut PCIDevice,
}

impl<'a> PCItoPCIBridgeDevice<'a> {
    pub fn fetch(dev: &'a mut PCIDevice) -> Self {
        Self { base: dev }
    }
}

#[derive(Debug)]
pub struct PCItoCardBusBridgeDevice<'a> {
    pub base: &'a mut PCIDevice,
}

impl<'a> PCItoCardBusBridgeDevice<'a> {
    pub fn fetch(dev: &'a mut PCIDevice) -> Self {
        Self { base: dev }
    }
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

unsafe fn pci_config_write_dword(bus: u8, slot: u8, func: u8, off: u8, val: u32) {
    outl(
        CONFIG_ADDRESS,
        ((bus as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | ((off as u32) & 0xFC)
            | 0x80000000u32,
    );

    outl(CONFIG_DATA, val);
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

                let _dword4 = pci_config_read_dword(bus, slot, func, 0x4);

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
                    bus,
                    slot,
                    func,
                    vendor,
                    device,
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
