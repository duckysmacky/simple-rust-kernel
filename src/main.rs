#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;
mod qemu;
mod testing;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit(qemu::QemuExitCode::Failed);

    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

fn main() {
    println!("Hello, world!");
}

#[test_case]
fn example_test() {
    assert_eq!(4, 2 * 2);
}