use crate::dprintln;
use crate::drv::io::ioport::outb;

pub unsafe fn mask_all() {
    outb(0x21, 0xFF);
    outb(0xA1, 0xFF);
    dprintln!("Initialized PIC");
}
