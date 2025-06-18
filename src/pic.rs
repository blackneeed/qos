use crate::io::{inb, outb};
use spin::Mutex;
use core::option::Option::{self, None};

pub struct PIC {
    offset_master: u8,
    offset_slave: u8,
}

impl PIC {
    pub const fn new() -> PIC
    {
        return PIC { offset_master: 0, offset_slave: 7 }; // x86 defaults
    }

    pub fn get_master_offset(&self) -> u8 {
        return self.offset_master;
    }

    pub fn get_slave_offset(&self) -> u8 {
        return self.offset_slave;
    }

    pub fn remap(&mut self, offset_master: u8, offset_slave: u8)
    {
        if self.offset_master == offset_master && self.offset_slave == offset_slave
        {
            return;
        }
        
        unsafe {
            // master
            outb(0x20, 0x11); // init
            outb(0x21, offset_master); // offset
            outb(0x21, 4); // irq2 = slave
            outb(0x21, 0x01); // 8086
            outb(0x21, 0xFF); // mask

            // slave
            outb(0xA0, 0x11); // init
            outb(0xA1, offset_slave); // offset
            outb(0xA1, 2); // identity
            outb(0xA1, 0x01); // 8086
            outb(0xA1, 0xFF); // mask
        }

        self.offset_master = offset_master;
        self.offset_slave = offset_slave;
    }

    pub fn eoi_master(&self)
    {
        unsafe { outb(0x20,0x20) };
    }

    pub fn eoi_slave(&mut self)
    {
        unsafe { outb(0xA0,0x20) };
        self.eoi_master();
    }

    fn read_mask(&self) -> u8
    {
        unsafe {
            return inb(0x21) | (inb(0xA1) << 4);
        }
    }

    fn write_mask(&self, mask: u8)
    {
        unsafe {
            outb(0x21, mask & 0xFF);
            outb(0xA1, mask >> 4);
        }
    }

    pub fn mask(&mut self, irq: u8)
    {
        if irq > 15
        {
            return;
        }

        self.write_mask(self.read_mask() | (1 << irq));
    }

    pub fn unmask(&mut self, irq: u8)
    {
        if irq > 15
        {
            return;
        }

        self.write_mask(self.read_mask() & !(1 << irq));
    }
}

pub static PIC_DRIVER: Mutex<Option<PIC>> = Mutex::new(None);