#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(improper_ctypes)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;
use qos_rust::disk::ATADrive;
use qos_rust::pic::{PIC, PIC_DRIVER};
use qos_rust::range::Range;
use qos_rust::multiboot::MultibootInfo;
use linked_list_allocator::LockedHeap;
use qos_rust::{println};
use qos_rust::panic::_hcf;
use qos_rust::idt::init_idt;
use qos_rust::mem::get_biggest_usable_pool;
use core::alloc::{GlobalAlloc, Layout};
use core::option::Option;
use core::arch::asm;
use fatfs::{FileSystem, FsOptions, Read};

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

    {
        let mut lock = PIC_DRIVER.lock();
        *lock = Some(PIC::new());
        let pic = lock.as_mut().unwrap();
        pic.remap(32, 40);

        println!("Initialized PIC! {:?}:{:?}", pic.get_master_offset(), pic.get_slave_offset());
    }

    for id in 0..4
    {
        match ATADrive::new(id) {
            Some(drive) => {
                println!("Initialized ATA PI/O disk {}.", id);
                if let Ok(fs) = FileSystem::new(drive, FsOptions::new())
                {
                    println!("Initialized FS on ATA PI/O disk {}.", id);

                    for i in fs.root_dir().iter()
                    {
                        println!("Enumerating root directory: ");
                        if let Ok(dir_ent) = i
                        {
                            println!("\t{}B {} {}", dir_ent.len(), match dir_ent.is_file() {
                                true => "FIL",
                                false => "DIR"
                            }, dir_ent.file_name());
                            if dir_ent.is_file()
                            {
                                if let Ok(mut file) = fs.root_dir().open_file(dir_ent.file_name().as_str())
                                {
                                    let buf = ALLOCATOR.alloc(Layout::from_size_align(dir_ent.len().try_into().expect("Could not convert dir_ent.len() into usize from u64."), 8).unwrap());
                                    if buf.is_null()
                                    {
                                        println!("Could not read file: alloc returned null");
                                    } else
                                    {
                                        let buf_slice = core::slice::from_raw_parts_mut(buf, dir_ent.len().try_into().expect("Could not convert dir_ent.len() into usize from u64."));
                                        if let Ok(read) = file.read(buf_slice)
                                        {
                                            if (read as u64) < dir_ent.len()
                                            {
                                                println!("Could not read file: read bytes {} < {}", read as u64, dir_ent.len())
                                            }
                                            println!("Contents: {}", core::str::from_utf8(buf_slice).expect("Could not get string from file contents"));
                                        } else {
                                            println!("Could not read file");
                                        }
                                    }
                                } else {
                                    println!("Could not read file");
                                }
                            }
                        }
                    }
                } else {
                    println!("Couldn't initialize FS on ATA PI/O disk {}.", id);
                }
            }
    
            None => {
                println!("Could not initialize ATA PI/O disk {}.", id);
            }
        }
    }

    loop { asm!("hlt") };
    //_hcf();
}