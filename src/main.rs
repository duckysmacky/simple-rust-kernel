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
    let color = ColorCode::new(Color::Yellow, Color::Black);
    let mut writer = vga_buffer::Writer::new(color, 0xb8000);

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}