use crate::kernel::{KERNEL_END, KERNEL_START};
use crate::multiboot::{
    MultibootInfo, MultibootInfoTag, MultibootMemoryMapEntry, MultibootMemoryMapTag,
};
use crate::range::{ChopResult, Range};
use core::cmp::min;
use core::option::Option::{self, None, Some};

pub unsafe fn get_memory_map_tag(
    mb2_info: *const MultibootInfo,
) -> Option<*const MultibootMemoryMapTag> {
    if (*mb2_info).total_size as usize == core::mem::size_of::<MultibootInfo>() {
        return None;
    }

    let mut tag_ptr = (&raw const (*mb2_info).tags) as *const MultibootInfoTag;
    loop {
        //println!("{:?}", *tag_ptr);
        if (*tag_ptr).type_ == 0 {
            return None;
        }

        if (*tag_ptr).type_ == 6 {
            return Some(tag_ptr as *const MultibootMemoryMapTag);
        }

        tag_ptr = (tag_ptr as *const u8).add(((*tag_ptr).size as usize + 7) & !7)
            as *const MultibootInfoTag;
    }
}

pub unsafe fn get_biggest_usable_pool(mmap_tag: *const MultibootMemoryMapTag) -> Option<Range> {
    let mut biggest_usable_memory_pool: Range = Range { start: 0, end: 0 };
    let mut entry = (&raw const (*mmap_tag).entries) as *const MultibootMemoryMapEntry;
    let mut size: u32 = 0;

    loop {
        if (*entry).base_addr > u32::MAX as u64 {
            entry = (entry as u32 + (((*mmap_tag).entry_size + 7) & !7))
                as *const MultibootMemoryMapEntry;
            size += ((*mmap_tag).entry_size + 7) & !7;

            if size >= (*mmap_tag).size - size_of::<MultibootMemoryMapTag>() as u32 {
                break;
            }

            continue;
        }

        if (*entry).type_ == 1
            && min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32
                > biggest_usable_memory_pool.end
        {
            biggest_usable_memory_pool.start = (*entry).base_addr as u32;
            biggest_usable_memory_pool.end =
                min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32;
        }

        entry =
            (entry as u32 + (((*mmap_tag).entry_size + 7) & !7)) as *const MultibootMemoryMapEntry;
        size += ((*mmap_tag).entry_size + 7) & !7;

        if size >= (*mmap_tag).size - size_of::<MultibootMemoryMapTag>() as u32 {
            break;
        }
    }

    let kernel = Range {
        start: (&KERNEL_START as *const ()).addr() as u32,
        end: (&KERNEL_END as *const ()).addr() as u32,
    };

    match biggest_usable_memory_pool.chop(kernel) {
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
