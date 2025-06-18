#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;
use qos_rust::range::Range;
use qos_rust::multiboot::MultibootInfo;
use linked_list_allocator::LockedHeap;
use qos_rust::println;
use qos_rust::panic::_hcf;
use qos_rust::idt::init_idt;
use qos_rust::mem::get_biggest_usable_pool;
use core::option::Option;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain(mb2_info: *const MultibootInfo) {
    let biggest_usable_memory_pool: Option<Range> = get_biggest_usable_pool(mb2_info);
    if biggest_usable_memory_pool.is_none()
    {
        println!("No usable memory pools!");
        _hcf();
    }

    ALLOCATOR.lock().init(biggest_usable_memory_pool.clone().unwrap().start as *mut u8, biggest_usable_memory_pool.clone().unwrap().end as usize - biggest_usable_memory_pool.clone().unwrap().start as usize);
    println!("Initialized allocator!");

    init_idt();
    println!("Initialized IDT!");

    _hcf();
}