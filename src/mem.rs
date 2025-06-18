use crate::range::{Range, ChopResult};
use crate::multiboot::{MultibootInfo, MultibootMemoryMapEntry};
use core::cmp::min;
use crate::kernel::{KERNEL_START, KERNEL_END};
use core::option::Option::{self, Some, None};

pub unsafe fn get_biggest_usable_pool(mb2_info: *const MultibootInfo) -> Option<Range>
{
    let mut biggest_usable_memory_pool: Range = Range { start: 0, end: 0 };
    for entry in core::slice::from_raw_parts((*mb2_info).mmap_addr as *const MultibootMemoryMapEntry, ((*mb2_info).mmap_length / size_of::<MultibootMemoryMapEntry>() as u32) as usize)
    {
        if (*entry).base_addr > u32::MAX as u64
        {
            continue;
        }

        if (*entry).type_ == 1 && min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32 > biggest_usable_memory_pool.end
        {
            biggest_usable_memory_pool.start = (*entry).base_addr as u32;
            biggest_usable_memory_pool.end = min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32;
        }
    }
    
    let kernel = Range { start: (&KERNEL_START as *const ()).addr() as u32, end: (&KERNEL_END as *const ()).addr() as u32 };
    
    match biggest_usable_memory_pool.chop(kernel)
    {
        ChopResult::One(r) => {
            biggest_usable_memory_pool = r;
        }
    
        ChopResult::Two(r1, r2) => {
            if r1.end - r1.start > r2.end - r2.start {
                biggest_usable_memory_pool = r1;
            } else {
                biggest_usable_memory_pool = r2;
            }
        }
    
        ChopResult::None => {
            return None;
        }
    }
    
    return Some(biggest_usable_memory_pool);
}
