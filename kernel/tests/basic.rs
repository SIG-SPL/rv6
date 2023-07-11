//! Tests for the test framework itself and the allocator.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

extern crate alloc;

#[no_mangle]
pub extern "C" fn os_main(_hartid: usize, dtb_pa: usize) -> ! {
    kernel::allocator::init();
    kernel::io::init(dtb_pa);
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

#[test_case]
fn test_virtio_blk_rw() {
    use kernel::io::virtio::block;
    let mut origin = alloc::vec![0x0; 512];
    let input = alloc::vec![0xffu8; 512];
    let mut output = alloc::vec![0; 512];
    block::read(0, &mut origin).unwrap();
    block::write(0, &input).unwrap();
    block::read(0, &mut output).unwrap();
    assert_eq!(input, output);
    block::write(0, &origin).unwrap();
}
