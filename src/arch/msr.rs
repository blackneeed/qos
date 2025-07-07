use core::arch::asm;

pub unsafe fn rdmsr(msr: u32) -> u64 {
    let o1: u32;
    let o2: u32;

    asm!("rdmsr", out("eax") o1, out("edx") o2, in("ecx") msr);
    ((o2 as u64) << 32) | (o1 as u64)
}

pub unsafe fn wrmsr(msr: u32, val: u64) {
    asm!("wrmsr", in("ecx") msr, in("eax") (val & 0xFFFFFFFF) as u32, in("edx") (val >> 32) as u32);
}

