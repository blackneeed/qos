use crate::dprintln;
use crate::initialized;
use crate::util::range::Range;
use core::alloc::GlobalAlloc;
use linked_list_allocator::LockedHeap;

#[allow(dead_code)]
pub struct WrappedHeap {
    pub under: LockedHeap,
}

#[allow(dead_code)]
impl WrappedHeap {
    pub const fn empty() -> WrappedHeap {
        WrappedHeap {
            under: LockedHeap::empty(),
        }
    }

    pub unsafe fn init(&self, start: *mut u8, size: usize) {
        self.under.lock().init(start, size);
    }
}

#[allow(dead_code)]
unsafe impl GlobalAlloc for WrappedHeap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ret = self.under.alloc(layout);
        dprintln!("Allocation {:?} -> {:#08X}", layout, ret as usize);
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        dprintln!("Deallocation {:#08X} {:?}", ptr as usize, layout);
        self.under.dealloc(ptr, layout)
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: core::alloc::Layout,
        new_size: usize,
    ) -> *mut u8 {
        let ret = self.under.realloc(ptr, layout, new_size);

        dprintln!(
            "Reallocation {:#08X} {:?} (new {}) -> {:#08X}",
            ptr as usize,
            layout,
            new_size,
            ret as usize
        );

        ret
    }

    unsafe fn alloc_zeroed(&self, layout: core::alloc::Layout) -> *mut u8 {
        dprintln!("Zeroed Allocation {:?}", layout);
        self.under.alloc_zeroed(layout)
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn initialize_allocator(pool: Range) {
    ALLOCATOR.lock().init(
        pool.start as *mut u8,
        pool.end as usize - pool.start as usize,
    );
    initialized!("MM");
}
