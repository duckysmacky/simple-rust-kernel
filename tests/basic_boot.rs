#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    kernel::hlt_loop();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}