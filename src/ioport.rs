use core::arch::asm;

pub unsafe fn inb(port: u16) -> u8 {
    let rv: u8;
    asm!("in al, dx", out("al") rv, in("dx") port);
    rv
}

pub unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("al") val, in("dx") port);
}

pub unsafe fn inw(port: u16) -> u16 {
    let rv: u16;
    asm!("in ax, dx", out("ax") rv, in("dx") port);
    rv
}

pub unsafe fn outw(port: u16, val: u16) {
    asm!("out dx, ax", in("ax") val, in("dx") port);
}

pub unsafe fn inl(port: u16) -> u32 {
    let rv: u32;
    asm!("in eax, dx", out("eax") rv, in("dx") port);
    rv
}

pub unsafe fn outl(port: u16, val: u32) {
    asm!("out dx, eax", in("eax") val, in("dx") port);
}
