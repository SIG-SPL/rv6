#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
#![test_runner(rv6::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use log::{error, info};
use rv6::{context, logging};

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    #[cfg(test)]
    test_main();

    logging::init();
    context::trap_init();
    info!("Hello, RV6!");
    panic!("It should shutdown!")
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }
    rv6::sbi::shutdown()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv6::test_panic_handler(info)
}
