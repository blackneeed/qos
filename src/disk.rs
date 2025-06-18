use crate::io::{inw, inb, outb};

pub struct CHS {
    cylinder: u16,
    head: u8,
    sector: u8
}

// i wrote this completely not knowing anything about ata pio but im not gonna nuke it ill use it sometime lmao
#[allow(dead_code)]
impl CHS {
    pub fn to_lba(&self, sectors_per_track: u8, heads: u8) -> u64
    {
        return (self.cylinder as u64 * heads as u64 + self.head as u64) * sectors_per_track as u64 + (self.sector as u64 - 1u64)
    }

    pub fn from_lba(lba: u64, sectors_per_track: u8, heads: u8) -> CHS
    {
        return CHS { cylinder: (lba / ((heads as u64) * (sectors_per_track as u64))) as u16, head: ((lba / (sectors_per_track as u64)) % (heads as u64)) as u8, sector: ((lba % (sectors_per_track as u64)) + 1) as u8 };
    }
}

pub struct ATADrive {
    io: u16,
    #[allow(dead_code)] // i guess we aint using it rn but it might be useful sometime? idk
    ctrl: u16,
    slave: bool,
    #[allow(dead_code)] // i guess we aint using it rn but it might be useful sometime? idk
    lba48: bool,
    max_lba: u64
}

impl ATADrive {
    pub fn new(drive_id: u8) -> Option<ATADrive> {
        let (io, ctrl, slave) = match drive_id {
            0 => (0x1F0, 0x3F6, false),
            1 => (0x1F0, 0x3F6, true),
            2 => (0x170, 0x376, false),
            3 => (0x170, 0x376, true),
            _ => return None,
        };

        unsafe {
            let sel = if slave { 0xB0 } else { 0xA0 };
            outb(io + 6, sel);
            for _ in 1..5 { inb(ctrl); } // delay
            outb(io + 7, 0xEC); // identify
    
            let mut stat = inb(io + 7);
            if stat == 0 { return None; }
            while stat & 0x80 != 0
            {
                stat = inb(io + 7);
            }
    
            if stat & 0x08 == 0
            {
                return None; // no drive
            }
    
            let mut buf = [0u16; 256];
            for i in 0..256 
            {
                buf[i] = inw(io);
            }
    
            if buf[49] & (1 << 9) == 0 
            {
                return None; // no lba support - fuck that
            }
    
            let lba48 = buf[83] & (1 << 10) != 0;
            let max_lba = if lba48
            {
                (buf[100] as u64)
                    | ((buf[101] as u64) << 16)
                    | ((buf[102] as u64) << 32)
                    | ((buf[103] as u64) << 48)
            } else
            {
                ((buf[61] as u32) << 16 | buf[60] as u32) as u64
            };

            Some(ATADrive { io: io, ctrl: ctrl, slave: slave, lba48: lba48, max_lba: max_lba })
        }
    }

    pub fn read_lba(&self, lba: u64, buffer: &mut [u8]) -> bool {
        if buffer.len() < 512
        {
            return false;
        }

        if lba >= self.max_lba
        {
            return false;
        }

        let sel = if self.slave { 0xF0 } else { 0xE0 } | ((lba >> 24) & 0x0F) as u8;
        unsafe {
            outb(self.io + 6, sel);
            outb(self.io + 2, 1); // count
            outb(self.io + 3, (lba & 0xFF) as u8);
            outb(self.io + 4, ((lba >> 8) & 0xFF) as u8);
            outb(self.io + 5, ((lba >> 16) & 0xFF) as u8);
            outb(self.io + 7, 0x20); // read
    
            let mut stat;
            loop {
                stat = inb(self.io + 7);
                if stat & 0x80 == 0 && stat & 0x08 != 0 {
                    break;
                }
            }
    
            for i in 0..256 {
                let data = inw(self.io);
                buffer[i * 2] = (data & 0xFF) as u8;
                buffer[i * 2 + 1] = (data >> 8) as u8;
            }
        }

        return true;
    }

    pub fn get_max_lba(&self) -> u64 {
        return self.max_lba;
    }
}