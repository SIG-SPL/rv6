#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod sbi;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("asm/start.S"));
global_asm!(include_str!("asm/trap.S"));
global_asm!(include_str!("asm/swich.S"));

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    println!("Hello, RV6!");
    panic!("It should shutdown!")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // print red error
    print!("\x1b[1;31mError: \x1b[0m");
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    crate::sbi::shutdown()
}
