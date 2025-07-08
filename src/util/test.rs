use core::alloc::Layout;

use crate::{dprint, dprintln, drv::io::ioport::outb, util::panic::_hcf};

pub struct Case {
    name: &'static str,
    func: &'static dyn Fn(),
}

unsafe impl Sync for Case {}

pub fn runner(tests: &[&Case]) {
    for i in tests {
        dprintln!("Running {} test", i.name);
        i.func.call(());
    }

    unsafe {
        outb(0x501, 0);
        _hcf();
    }
}

#[test_case]
static ALLOCATOR: Case = Case {
    name: "allocator + memory",
    func: &allocator_test,
};

pub fn allocator_test() {
    unsafe {
        let sizes: [usize; 24] = [
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608,
        ];

        let mut layouts: [Layout; 24] = [Layout::new::<()>(); 24];

        assert_eq!(sizes.len(), layouts.len(), "check allocator_test");

        for i in 0..layouts.len() {
            layouts[i] = Layout::from_size_align(sizes[i], 1)
                .expect("could not create layout in allocator_test");
        }

        const ITERS: u8 = 2;

        for i in 0..ITERS {
            dprint!("[{}/{}] [", i + 1, ITERS);

            for (j, &l) in layouts.iter().enumerate() {
                let ptr = alloc::alloc::alloc(l);
                let mut index = 0usize;
                while index < sizes[j] {
                    let wrapped = index.wrapping_mul(0x9E3779B1);
                    *ptr.add(index) = ((wrapped & 0xFF) as u8) ^ (((wrapped >> 8) & 0xFF) as u8);
                    index += (index % 16).max(1);
                }

                for _ in 0..1000000 {} // small delay

                let mut index = 0usize;
                while index < sizes[j] {
                    let wrapped = index.wrapping_mul(0x9E3779B1);
                    if *ptr.add(index) != ((wrapped & 0xFF) as u8) ^ (((wrapped >> 8) & 0xFF) as u8)
                    {
                        panic!("invalid val found");
                    }
                    index += (index % 16).max(1);
                }

                alloc::alloc::dealloc(ptr, layouts[j]);
                dprint!("{}{}", if j > 0 { ", " } else { "" }, sizes[j]);
            }

            dprintln!("]");
        }
    }
}
