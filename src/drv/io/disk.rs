use alloc::boxed::Box;
use alloc::vec::Vec;
use spin::Mutex;

pub struct CHS {
    cylinder: u16,
    head: u8,
    sector: u8,
}

impl CHS {
    pub fn to_lba(&self, sectors_per_track: u8, heads: u8) -> u64 {
        (self.cylinder as u64 * heads as u64 + self.head as u64) * sectors_per_track as u64
            + (self.sector as u64 - 1u64)
    }

    pub fn from_lba(lba: u64, sectors_per_track: u8, heads: u8) -> CHS {
        CHS {
            cylinder: (lba / ((heads as u64) * (sectors_per_track as u64))) as u16,
            head: ((lba / (sectors_per_track as u64)) % (heads as u64)) as u8,
            sector: ((lba % (sectors_per_track as u64)) + 1) as u8,
        }
    }
}

pub trait Disk {
    fn write(&mut self, data: &[u8; 512]) -> u64;
    fn read(&mut self, buf: &mut [u8; 512]) -> u64;
    fn seek(&mut self, seek: DiskSeek) -> Result<u64, ()>;
}

pub enum DiskSeek {
    End(u64),
    Start(u64),
    Current(u64),
}

static DISKS: Mutex<Vec<Box<dyn Disk + Send>>> = Mutex::new(Vec::new());

pub fn register_disk(dsk: Box<dyn Disk + Send>) {
    DISKS.lock().push(dsk);
}
