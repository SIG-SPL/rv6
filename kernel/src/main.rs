#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::vec;
use core::{arch::asm, panic::PanicInfo};
use kernel::{allocator, logging, trap};

extern crate alloc;

/// TODO: set this function to test.
#[no_mangle]
pub fn loop_print() -> ! {
    let string = "Hello, world!\n";
    loop {
        let mut ret = 0;
        unsafe {
            asm!(
                "ecall",
                inlateout("a0") 1 => ret,
                in("a1") string.as_ptr(),
                in("a2") string.len(),
                in("a7") 64,
            );
        }
        assert_eq!(ret, 14);
    }
}

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    #[cfg(test)]
    test_main();

    trap::init();
    logging::init();
    allocator::init();

    unsafe {
        // set sepc to loop_print and sret
        riscv::register::sepc::write(loop_print as usize);
        asm!("sret")
    }
    panic!("Unreachable in os_main!")
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log::error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        log::error!("Panicked: {}", info.message().unwrap());
    }
    kernel::sbi::shutdown()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
