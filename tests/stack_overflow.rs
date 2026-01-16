#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use kernel::{serial_print, serial_println};

lazy_static::lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(kernel::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel::gdt::init_gdt();
    init_test_idt();

    test_should_overflow();

    panic!("Execution continued after stack overflow");
}

fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    use kernel::qemu;
    serial_println!("[ok]");
    qemu::exit(qemu::QemuExitCode::Success);
    loop {}
}

fn test_should_overflow() {
    serial_print!("stack_overflow::should_overflow...\t");

    #[allow(unconditional_recursion)]
    fn recursive() {
        recursive();
        volatile::Volatile::new(0).read();
    }

    recursive();
}
