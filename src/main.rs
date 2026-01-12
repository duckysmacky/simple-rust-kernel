#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel::init();

    main();
    #[cfg(test)]
    test_main();

    loop {}
}

fn main() {
    println!("Hello, world!");
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
