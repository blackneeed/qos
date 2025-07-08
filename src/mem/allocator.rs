use crate::initialized;
use crate::util::range::Range;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn initialize_allocator(pool: Range) {
    ALLOCATOR.lock().init(
        pool.start as *mut u8,
        pool.end as usize - pool.start as usize,
    );
    initialized!("MM");
}
