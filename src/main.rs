#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;
use core::arch::asm;
use qos_rust::range::{Range, ChopResult};
use qos_rust::multiboot::{MultibootInfo, MultibootMemoryMapEntry};
use core::cmp::min;
use linked_list_allocator::LockedHeap;
use alloc::vec::Vec;
use qos_rust::println;
use qos_rust::panic::_hcf;
use qos_rust::idt::init_idt;

unsafe extern "C" {
    static KERNEL_START: ();
    static KERNEL_END: ();
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    let mut biggest_usable_memory_pool: Range = Range { start: 0, end: 0 };
    for entry in core::slice::from_raw_parts((*mb2_info).mmap_addr as *const MultibootMemoryMapEntry, ((*mb2_info).mmap_length / size_of::<MultibootMemoryMapEntry>() as u32) as usize)
    {
        if (*entry).base_addr > u32::MAX as u64
        {
            // println!("block {:#016X}-{:#016X}:", (*entry).base_addr, (*entry).base_addr + (*entry).length);
            // println!("\tstart address {:#016X} is bigger than 32-bit max ({:#016X}), block is unusable.", (*entry).base_addr, u32::MAX);
            continue;
        }/* else if (*entry).base_addr + (*entry).length > u32::MAX as u64 {
            // println!("block {:#016X}-{:#016X}:", (*entry).base_addr, (*entry).base_addr + (*entry).length);
            // println!("\tend address {:#016X} is bigger than 32-bit max ({:#016X}), block needs to be chopped down to {:#016X}-{:#016X} from {:#016X}-{:#016X}.", (*entry).base_addr + (*entry).length, u32::MAX, (*entry).base_addr, u32::MAX, (*entry).base_addr, (*entry).base_addr + (*entry).length);
        }*/

        /*  let type_str: &str = match (*entry).type_ {
             1 => "Available",
             2 => "Reserved",
             3 => "ACPI",
             4 => "NVS",
             5 => "Defective RAM",
             _ => "Unknown",
         }; */

        // print!("{:#016X}-{:#016X} ({})", (*entry).base_addr as u32, min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32, type_str);
        if (*entry).type_ == 1 && min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32 > biggest_usable_memory_pool.end
        {
            biggest_usable_memory_pool.start = (*entry).base_addr as u32;
            biggest_usable_memory_pool.end = min((*entry).base_addr + (*entry).length, u32::MAX as u64) as u32;
            // print!(" (promoted to new biggest usable pool)");
        }
        // println!();
    }

    let kernel = Range { start: (&KERNEL_START as *const ()).addr() as u32, end: (&KERNEL_END as *const ()).addr() as u32 };
    // println!("kernel: {:#016X}-{:#016X}", kernel.start, kernel.end);

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
            println!("\tNo usable memory pools, halting");
            _hcf();
        }
    }

    println!("Biggest usable memory pool: {:#016X}-{:#016X}", biggest_usable_memory_pool.start, biggest_usable_memory_pool.end);
    ALLOCATOR.lock().init(biggest_usable_memory_pool.start as *mut u8, biggest_usable_memory_pool.end as usize - biggest_usable_memory_pool.start as usize);
    println!("Initialized allocator!");
    let mut vec = Vec::new();

    for i in 1..500 {
        vec.push(i);
    }

    for i in 1..500 {
        if vec.pop().unwrap() != 500 - i
        {
            println!("fail");
        }
    }

    println!("Allocator passed test!");
    init_idt();

    asm!(
        "xor edx, edx
        xor eax, eax
        div edx"
    );

    _hcf();
}