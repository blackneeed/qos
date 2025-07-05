use crate::kprintln;
use core::arch::asm;
use core::panic::PanicInfo;

pub unsafe fn _hcf() -> ! {
    asm!("cli");
    loop {
        asm!("hlt");
    }
}

pub unsafe fn infhlt() -> ! {
    loop {
        asm!("hlt");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintln!("{}", _info.message());
    unsafe {
        _hcf();
    }
}
