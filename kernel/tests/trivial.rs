//! Tests for the test framework itself and the allocator.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;
use kernel::allocator;

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    allocator::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1 + 1, 2);
}

#[test_case]
fn test_alloc() {
    use alloc::boxed::Box;
    use alloc::vec;
    let heap_val = Box::new(41);
    assert_eq!(*heap_val, 41);
    let vec = vec![1, 2, 3];
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}
