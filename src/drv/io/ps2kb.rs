use crate::drv::io::ioport::inb;
use crate::drv::io::ps2::PS2_DATA_PORT;

pub unsafe fn ps2_keyboard_irq() {
    let _scancode = inb(PS2_DATA_PORT);
}
