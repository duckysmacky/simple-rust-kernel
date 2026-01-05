#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga_buffer::{ColorCode, Color};

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_something();

    loop {}
}

fn print_something() {
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();

    writer.write_byte(b'H');
    writer.write_string("ello, World!\n");
    write!(writer, "The numers are {} and {}", 42, 1.5).unwrap();
}