use crate::ioport::{inb, inw, outb};
use fatfs::{IoBase, Read, Seek, SeekFrom, Write};

pub struct ATADrive {
    io: u16,
    #[allow(dead_code)] // i guess we aint using it rn but it might be useful sometime? idk
    ctrl: u16,
    slave: bool,
    max_lba: u64,
    position: u64, // for write and read and seek traits
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
            for _ in 1..5 {
                inb(ctrl);
            } // delay
            outb(io + 7, 0xEC); // identify

            let mut stat = inb(io + 7);
            if stat == 0 {
                return None;
            }
            while stat & 0x80 != 0 {
                stat = inb(io + 7);
            }

            if stat & 0x08 == 0 {
                return None; // no drive
            }

            let mut buf = [0u16; 256];

            for (i, _) in buf.clone().iter().enumerate() {
                buf[i] = inw(io);
            }

            if buf[49] & (1 << 9) == 0 {
                return None; // no lba support - fuck that
            }

            let max_lba = ((buf[61] as u32) << 16 | buf[60] as u32) as u64;

            Some(ATADrive {
                io,
                ctrl,
                slave,
                max_lba,
                position: 0,
            })
        }
    }

    pub fn read_lba(&self, lba: u64, buffer: &mut [u8]) -> bool {
        if buffer.len() < 512 {
            return false;
        }

        if lba >= self.max_lba {
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

        true
    }

    fn write_lba(&self, lba: u64, data: &[u8]) -> bool {
        if data.len() < 512 || lba >= self.max_lba {
            return false;
        }

        unsafe {
            let sel = if self.slave { 0xF0 } else { 0xE0 } | ((lba >> 24) & 0x0F) as u8;
            outb(self.io + 6, sel);
            outb(self.io + 2, 1);
            outb(self.io + 3, (lba & 0xFF) as u8);
            outb(self.io + 4, ((lba >> 8) & 0xFF) as u8);
            outb(self.io + 5, ((lba >> 16) & 0xFF) as u8);
            outb(self.io + 7, 0x30); // write

            loop {
                let stat = inb(self.io + 7);
                if stat & 0x80 == 0 && stat & 0x08 != 0 {
                    break;
                }
            }

            for i in 0..256 {
                let word = data[i * 2] as u16 | ((data[i * 2 + 1] as u16) << 8);
                outb(self.io, (word & 0xFF) as u8);
                outb(self.io, (word >> 8) as u8);
            }
        }

        true
    }

    pub fn get_max_lba(&self) -> u64 {
        self.max_lba
    }
}

impl IoBase for ATADrive {
    type Error = ();
}

impl Read for ATADrive {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut total_read = 0;
        let mut remaining = buf;

        while !remaining.is_empty() {
            let lba = self.position / 512;
            let sector_offset = (self.position % 512) as usize;

            if lba >= self.max_lba {
                break;
            }

            let mut sector = [0u8; 512];
            if !self.read_lba(lba, &mut sector) {
                break;
            }

            let copy_len = (512 - sector_offset).min(remaining.len());
            remaining[..copy_len].copy_from_slice(&sector[sector_offset..sector_offset + copy_len]);

            self.position += copy_len as u64;
            total_read += copy_len;
            remaining = &mut remaining[copy_len..];
        }

        Ok(total_read)
    }
}

impl Write for ATADrive {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut total_written = 0;
        let mut remaining = buf;

        while !remaining.is_empty() {
            let lba = self.position / 512;
            let sector_offset = (self.position % 512) as usize;

            if lba >= self.max_lba {
                break;
            }

            let mut sector = [0u8; 512];
            if sector_offset != 0 || remaining.len() < 512 && !self.read_lba(lba, &mut sector) {
                break;
            }

            let copy_len = (512 - sector_offset).min(remaining.len());
            sector[sector_offset..sector_offset + copy_len].copy_from_slice(&remaining[..copy_len]);

            if !self.write_lba(lba, &sector) {
                break;
            }

            self.position += copy_len as u64;
            total_written += copy_len;
            remaining = &remaining[copy_len..];
        }

        Ok(total_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Seek for ATADrive {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        self.position = match pos {
            SeekFrom::Start(offset) => offset,
            SeekFrom::Current(offset) => (self.position as i64 + offset).max(0) as u64,
            SeekFrom::End(offset) => {
                let size = self.max_lba * 512;
                (size as i64 + offset).max(0) as u64
            }
        };

        Ok(self.position)
    }
}
