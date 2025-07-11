use spin::Mutex;

use crate::drv::io::pci::{DetailedPCIDevice, GeneralPCIDevice, PCI_DEVICES, PCIBar, PCIDevice};
use crate::kprint;
use crate::kprintln;
use crate::util::range::BoundExpect;
use core::ptr::read_volatile;

static RTL8139: Mutex<Option<RTL8139>> = Mutex::new(None);

struct RTL8139AccessMechanism<'a> {
    bar: &'a PCIBar,
}

impl<'a> RTL8139AccessMechanism<'a> {
    pub fn read8(&self, off: usize) -> u8 {
        0
    }
    pub fn read16(&self, off: usize) -> u16 {
        0
    }
    pub fn read32(&self, off: usize) -> u32 {
        0
    }
    pub fn read64(&self, off: usize) -> u64 {
        0
    }

    pub fn write8(&self, off: usize, val: u8) {}
    pub fn write16(&self, off: usize, val: u16) {}
    pub fn write32(&self, off: usize, val: u32) {}
    pub fn write64(&self, off: usize, val: u64) {}
}

pub struct RTL8139<'a> {
    pub access: RTL8139AccessMechanism<'a>,
    pub device: GeneralPCIDevice<'a>,
}

impl<'a> RTL8139<'a> {}

pub unsafe fn rtl8139_init() {
    if let Some(dev) = PCI_DEVICES
        .lock()
        .iter_mut()
        .find(|x| x.vendor == 0x10ec && x.device == 0x8139)
    {
        let dev = dev.to_detailed_device();
        let access: RTL8139AccessMechanism;
        if let DetailedPCIDevice::General(dev) = dev {
            if let Some(bar) = [
                &dev.bar0, &dev.bar1, &dev.bar2, &dev.bar3, &dev.bar4, &dev.bar5,
            ]
            .iter()
            .find(|x| matches!(x, PCIBar::Memory(_)))
            {
                access = RTL8139AccessMechanism { bar };
            } else if let Some(bar) = [
                &dev.bar0, &dev.bar1, &dev.bar2, &dev.bar3, &dev.bar4, &dev.bar5,
            ]
            .iter()
            .find(|x| matches!(x, PCIBar::IO(_)))
            {
                access = RTL8139AccessMechanism { bar };
            } else {
                return;
            }

            access.read8(0);
        }
    }

    //kprintln!("RTL8139:");
    //kprint!("\tEnabling Bus Mastering: ");
    //rtl.base.write_command(rtl.base.read_command() | (1 << 2));
    //kprintln!("done");
    //kprintln!(
    //    "\tIO base address: {:#04X}",
    //    io_bar.address.expect_bound::<u16>()
    //);
    //kprintln!(
    //    "\tMemory base address: {:#08X}",
    //    mem_bar.address.expect_bound::<u32>()
    //);
    //let ptr = mem_bar.address.expect_bound::<u32>() as *mut u8;
    //kprintln!(
    //    "\tMAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
    //    read_volatile(ptr),
    //    read_volatile(ptr.add(1)),
    //    read_volatile(ptr.add(2)),
    //    read_volatile(ptr.add(3)),
    //    read_volatile(ptr.add(4)),
    //    read_volatile(ptr.add(5))
    //);
}
