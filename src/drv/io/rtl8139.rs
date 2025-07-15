use crate::arch::util::pause;
use crate::drv::io::ioport::{inb, inl, inw, outb, outl, outw};
use crate::drv::io::pci::{DetailedPCIDevice, GeneralPCIDevice, PCI_DEVICES, PCIBar};
use crate::kprintln;
use crate::util::range::BoundExpect;
use alloc::boxed::Box;
use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

static RTL8139: Mutex<Option<RTL8139>> = Mutex::new(None);

#[derive(Debug, Clone)]
pub struct RTL8139AccessMechanism<'a> {
    bar: &'a PCIBar,
}

type RXBuffer = [u8; 8 * 1024 + 16];
pub struct MAC(u64);

impl MAC {
    pub fn from_parts((m1, m2, m3, m4, m5, m6): (u8, u8, u8, u8, u8, u8)) -> MAC {
        MAC(u64::from_le_bytes([0, 0, m1, m2, m3, m4, m5, m6]))
    }
}

impl Debug for MAC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytrs = self.0.to_le_bytes();
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            bytrs[2], bytrs[3], bytrs[4], bytrs[5], bytrs[6], bytrs[7]
        )
    }
}

impl<'a> RTL8139AccessMechanism<'a> {
    pub fn read8(&self, off: usize) -> u8 {
        if let PCIBar::IO(bar) = self.bar {
            unsafe { inb((bar.address + off.expect_bound::<u32>()).expect_bound::<u16>()) }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe { read_volatile((bar.address + off.expect_bound::<u32>()) as *const u8) }
        } else {
            0
        }
    }

    pub fn read16(&self, off: usize) -> u16 {
        if let PCIBar::IO(bar) = self.bar {
            unsafe { inw((bar.address + off.expect_bound::<u32>()).expect_bound::<u16>()) }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe { read_volatile((bar.address + off.expect_bound::<u32>()) as *const u16) }
        } else {
            0
        }
    }

    pub fn read32(&self, off: usize) -> u32 {
        if let PCIBar::IO(bar) = self.bar {
            unsafe { inl((bar.address + off.expect_bound::<u32>()).expect_bound::<u16>()) }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe { read_volatile((bar.address + off.expect_bound::<u32>()) as *const u32) }
        } else {
            0
        }
    }

    pub fn write8(&self, off: usize, val: u8) {
        if let PCIBar::IO(bar) = self.bar {
            unsafe {
                outb(
                    (bar.address + off.expect_bound::<u32>()).expect_bound::<u16>(),
                    val,
                );
            }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe {
                write_volatile((bar.address + off.expect_bound::<u32>()) as *mut u8, val);
            }
        }
    }

    pub fn write16(&self, off: usize, val: u16) {
        if let PCIBar::IO(bar) = self.bar {
            unsafe {
                outw(
                    (bar.address + off.expect_bound::<u32>()).expect_bound::<u16>(),
                    val,
                );
            }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe {
                write_volatile((bar.address + off.expect_bound::<u32>()) as *mut u16, val);
            }
        }
    }

    pub fn write32(&self, off: usize, val: u32) {
        if let PCIBar::IO(bar) = self.bar {
            unsafe {
                outl(
                    (bar.address + off.expect_bound::<u32>()).expect_bound::<u16>(),
                    val,
                );
            }
        } else if let PCIBar::Memory(bar) = self.bar {
            unsafe {
                write_volatile((bar.address + off.expect_bound::<u32>()) as *mut u32, val);
            }
        }
    }
}

#[derive(Debug)]
pub struct RTL8139<'a> {
    pub access: RTL8139AccessMechanism<'a>,
    device: GeneralPCIDevice<'a>,
    buffer: MaybeUninit<Box<RXBuffer>>,
}

impl<'a> RTL8139<'a> {
    pub fn new(pci: GeneralPCIDevice) -> Option<RTL8139> {
        let access: RTL8139AccessMechanism;
        let bars = Box::leak(Box::new([
            pci.bar0.clone(),
            pci.bar1.clone(),
            pci.bar2.clone(),
            pci.bar3.clone(),
            pci.bar4.clone(),
            pci.bar5.clone(),
        ]));

        if let Some(bar) = bars.iter().find(|x| matches!(x, PCIBar::Memory(_))) {
            access = RTL8139AccessMechanism { bar };
        } else if let Some(bar) = bars.iter().find(|x| matches!(x, PCIBar::IO(_))) {
            access = RTL8139AccessMechanism { bar };
        } else {
            return None;
        }

        let rtl = RTL8139 {
            access: access.clone(),
            device: pci.clone(),
            buffer: MaybeUninit::uninit(),
        };

        rtl.init();
        Some(rtl)
    }

    pub fn init(&self) {
        self.enable_bus_mastering();
        self.enable();
        self.reset();
        self.alloc_rx_buffer();
        self.set_rx_buffer();

        kprintln!("{:?}", self.get_mac());
    }

    pub fn enable(&self) {
        self.access.write8(0x52, 0);
    }

    pub fn reset(&self) {
        self.access.write8(0x37, 0x10);
        while (self.access.read8(0x37) & 0x10) != 0 {
            pause();
        }
    }

    pub fn enable_bus_mastering(&self) {
        self.device
            .base
            .write_command(self.device.base.read_command() | (1 << 2));
    }

    pub fn alloc_rx_buffer(&self) {
        self.buffer.write(Box::new(RXBuffer));
    }

    pub fn set_rx_buffer(&self) {}

    pub fn get_mac(&self) -> MAC {
        MAC::from_parts((
            self.access.read8(0),
            self.access.read8(1),
            self.access.read8(2),
            self.access.read8(3),
            self.access.read8(4),
            self.access.read8(5),
        ))
    }
}

pub unsafe fn rtl8139_init() {
    let dev = Box::leak(Box::new(PCI_DEVICES.lock().clone()))
        .iter_mut()
        .find(|x| x.vendor == 0x10ec && x.device == 0x8139);

    if let Some(dev) = dev
        && let DetailedPCIDevice::General(dev) = dev.to_detailed_device()
    {
        *RTL8139.lock() = RTL8139::new(dev);
    }
}
