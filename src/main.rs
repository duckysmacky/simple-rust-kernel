#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use kernel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;
    use kernel::{memory, allocator};

    kernel::init();

    let memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut memory_mapper = unsafe { memory::get_memory_mapper(memory_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::new(&boot_info.memory_map) };

    allocator::init_heap(&mut memory_mapper, &mut frame_allocator).expect("Heap initialization failed");

    #[cfg(test)]
    test_main();
    kernel::hlt_loop();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
