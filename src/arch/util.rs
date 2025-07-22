use core::arch::asm;

pub fn pause() {
    unsafe { asm!("pause"); }
}
