#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::{allocator, logging, task, trap, virtio};

extern crate alloc;

#[no_mangle]
pub extern "C" fn os_main(hartid: usize, dtb_pa: usize) -> ! {
    #[cfg(test)]
    test_main();

    trap::init(hartid);
    logging::init();
    allocator::init();
    virtio::init(dtb_pa);
    log::info!("Initialized hart {}", hartid);
    task::init();
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
    loop {}
    // kernel::sbi::shutdown()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
