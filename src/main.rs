#![no_std]
#![no_main]
#![allow(clippy::while_immutable_condition)]

mod device;

use core::arch::global_asm;
use core::panic::PanicInfo;

static HELLO: &str = "Hello RV6!\n";

global_asm!(include_str!("asm/start.S"));
global_asm!(include_str!("asm/trap.S"));
global_asm!(include_str!("asm/swich.S"));

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    print!("{}", HELLO);
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
