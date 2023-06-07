#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod context;
mod logging;
mod sbi;

#[macro_use]
extern crate log;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("asm/start.S"));
global_asm!(include_str!("asm/trap.S"));
global_asm!(include_str!("asm/swich.S"));

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    logging::init();
    context::trap_init();
    info!("Hello, RV6!");
    panic!("It should shutdown!")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print red error
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
    crate::sbi::shutdown()
}
