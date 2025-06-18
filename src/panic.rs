use core::panic::PanicInfo;
use core::arch::asm;

pub unsafe fn _hcf() -> !
{
    asm!("cli");
    loop {asm!("hlt");}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {_hcf();}
}