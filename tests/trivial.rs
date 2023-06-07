//! Tests for the test framework itself.
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rv6::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv6::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_assertion_failed() {
    assert_eq!(1, 2);
}
