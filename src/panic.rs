use core::panic::PanicInfo;
use core::arch::asm;
use crate::println;

pub unsafe fn _hcf() -> !
{
    asm!("cli");
    loop {asm!("hlt");}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info.message());
    unsafe {_hcf();}
}