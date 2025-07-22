use crate::arch::util::pause;
use crate::dprintln;
use crate::drv::io::ioport::{inb, inl, inw, outb, outl, outw};
use crate::drv::io::pci::{DetailedPCIDevice, GeneralPCIDevice, PCI_DEVICES, PCIBar};
use crate::initialized;
use crate::tables::idt::register_irq;
use crate::util::range::BoundExpect;
use alloc::boxed::Box;
use core::fmt::Debug;
use core::mem::MaybeUninit;
use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

static RTL8139: Mutex<Option<RTL8139>> = Mutex::new(None);

const MAC0_REGISTER: usize = 0x00;
const MAC1_REGISTER: usize = 0x01;
const MAC2_REGISTER: usize = 0x02;
const MAC3_REGISTER: usize = 0x03;
const MAC4_REGISTER: usize = 0x04;
const MAC5_REGISTER: usize = 0x05;
const RBSTART_REGISTER: usize = 0x30;
const COMMAND_REGISTER: usize = 0x37;
const CONFIG1_REGISTER: usize = 0x52;
const IMR_REGISTER: usize = 0x3C;
const ISR_REGISTER: usize = 0x3E;
const RCR_REGISTER: usize = 0x44;

const RX_BUFFER_SIZE: usize = 8 * 1024 + 16;

#[derive(Debug, Clone)]
pub struct RTL8139AccessMechanism<'a> {
    bar: &'a PCIBar,
}

type RXBuffer = [u8; RX_BUFFER_SIZE];
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
    rx_buffer: MaybeUninit<Box<RXBuffer>>,
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

        let mut rtl = RTL8139 {
            access: access.clone(),
            device: pci.clone(),
            rx_buffer: MaybeUninit::uninit(),
        };

        rtl.init();
        Some(rtl)
    }

    pub fn init(&mut self) {
        self.enable_bus_mastering();
        self.enable();
        self.reset();
        self.alloc_rx_buffer();
        self.set_rx_buffer();

        // bit 0 = rok
        // bit 2 = tok
        self.access.write16(IMR_REGISTER, (1 << 0) | (1 << 2)); // ROK+TOK

        // bit 11-12 = rblen
        // bit 7 = wrap
        // bit 3 = ab
        // bit 2 = am
        // bit 1 = apm
        // bit 0 = aap
        self.access.write32(
            RCR_REGISTER,
            (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (0 << 11),
        ); // AB+AM+APM+AAP RBLEN=00 (8k+16)

        // bit 3 = re
        // bit 2 = te
        self.access.write8(COMMAND_REGISTER, (1 << 3) | (1 << 2)); // RE+TE

        unsafe {
            register_irq(self.device.interrupt_line, |_| {
                RTL8139
                    .lock()
                    .as_ref()
                    .expect("No RTL8139 when RTL8139 IRQ triggered")
                    .irq();
            });
        }
    }

    pub fn enable(&self) {
        self.access.write8(CONFIG1_REGISTER, 0);
    }

    pub fn reset(&self) {
        // bit 4 = rst
        self.access.write8(COMMAND_REGISTER, 1 << 4);
        while (self.access.read8(COMMAND_REGISTER) & (1 << 4)) != 0 {
            // when the reset is
            // finished, the rst bit is
            // set low
            pause();
        }
    }

    pub fn enable_bus_mastering(&self) {
        self.device
            .base
            .write_command(self.device.base.read_command() | (1 << 2));
    }

    pub fn alloc_rx_buffer(&mut self) {
        self.rx_buffer.write(Box::new([0; RX_BUFFER_SIZE]));
    }

    pub fn set_rx_buffer(&self) {
        self.access.write32(
            RBSTART_REGISTER,
            (self.rx_buffer.as_ptr() as usize).expect_bound(),
        );
    }

    pub fn get_mac(&self) -> MAC {
        MAC::from_parts((
            self.access.read8(MAC0_REGISTER),
            self.access.read8(MAC1_REGISTER),
            self.access.read8(MAC2_REGISTER),
            self.access.read8(MAC3_REGISTER),
            self.access.read8(MAC4_REGISTER),
            self.access.read8(MAC5_REGISTER),
        ))
    }

    pub unsafe fn irq(&self) {
        dprintln!("RTL8139 IRQ");
        // bit 0 = rok
        // bit 2 = tok
        let status = self.access.read16(ISR_REGISTER);

        if (status & (1 << 0)) != 0 {
            dprintln!("Reason: ROK");
        }

        if (status & (1 << 2)) != 0 {
            dprintln!("Reason: TOK");
        }
    }
}

pub unsafe fn rtl8139_init() {
    let rtl8139 = Box::leak(Box::new(PCI_DEVICES.lock().clone()))
        .iter_mut()
        .find(|x| x.device == 0x8139 && x.vendor == 0x10ec);
    if let Some(dev) = rtl8139
        && let DetailedPCIDevice::General(dev) = dev.to_detailed_device()
    {
        *RTL8139.lock() = RTL8139::new(dev);
        initialized!("RTL8139");
    }
}
