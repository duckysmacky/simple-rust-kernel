#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel::qemu::{self, QemuExitCode};
use kernel::{serial_println, serial_print};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(QemuExitCode::Success);
    kernel::hlt_loop();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_should_fail();
    serial_println!("[test did not panic]");
    qemu::exit(QemuExitCode::Failed);
    kernel::hlt_loop();
}

fn test_should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}