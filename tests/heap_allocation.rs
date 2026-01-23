#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use kernel::{allocator, memory};
    use x86_64::VirtAddr;

    kernel::init();

    let memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::get_memory_mapper(memory_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();

    test_main();
    kernel::hlt_loop();
}

#[test_case]
fn test_box() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);

    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn test_vec() {
    let n = 100;
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i);
    }

    let sum = vec.iter().sum::<u64>();

    assert_eq!(sum, (n - 1) * n / 2);
}

#[test_case]
fn test_many_boxes() {
    for i in 0..kernel::allocator::HEAP_SIZE {
        let x = Box::new(i);

        assert_eq!(*x, i);
    }
}