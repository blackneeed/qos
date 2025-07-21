use crate::dprintln;
use crate::drv::io::ioport::outl;
use crate::util::panic::_hcf;
use core::alloc::Layout;

pub struct Case {
    name: &'static str,
    func: &'static dyn Fn() -> Result<(), &'static str>,
}

unsafe impl Sync for Case {}

pub fn runner(tests: &[&Case]) {
    let mut failed = 0usize;

    dprintln!("Running tests");

    for i in tests {
        if let Err(reason) = i.func.call(()) {
            dprintln!("{} failed: {reason}", i.name);
            failed += 1;
        } else {
            dprintln!("{} passed", i.name);
        }
    }

    dprintln!(
        "{} tests passed, {} tests failed",
        tests.len() - failed,
        failed
    );

    unsafe {
        _hcf();
    }
}

#[test_case]
static ALLOCATOR: Case = Case {
    name: "allocator",
    func: &allocator_test,
};

pub fn allocator_test() -> Result<(), &'static str> {
    unsafe {
        let sizes: [usize; 24] = [
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608,
        ];

        let mut layouts: [Layout; 24] = [Layout::new::<()>(); 24];

        if sizes.len() != layouts.len() {
            return Err("Size of sizes != Size of layouts");
        }

        for i in 0..layouts.len() {
            if let Ok(layout) = Layout::from_size_align(sizes[i], 1) {
                layouts[i] = layout;
            } else {
                return Err("Could not create layout");
            }
        }

        for (j, &l) in layouts.iter().enumerate() {
            let ptr = alloc::alloc::alloc(l);

            if ptr.is_null() {
                return Err("Allocator returned null");
            }

            alloc::alloc::dealloc(ptr, layouts[j]);
        }
        Ok(())
    }
}
