use core::arch::asm;

pub struct Core {
    pub apic_id: u8
}

impl Core {
    pub unsafe fn this() -> Core {
        let out: u8;
        asm!("cpuid
              shl ebx, 24", in("eax") 1, out("bl") out);
        Core { apic_id: out }
    }
}
