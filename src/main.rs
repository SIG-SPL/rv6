#![no_std]
#![no_main]
#![allow(clippy::while_immutable_condition)]

use core::arch::global_asm;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello RV6!\n";

global_asm!(include_str!("asm/start.S"));
global_asm!(include_str!("asm/trap.S"));
global_asm!(include_str!("asm/swich.S"));

pub const UART: usize = 0x1000_0000;
pub const UART_THR: *mut u8 = UART as *mut u8;
pub const UART_LSR: *mut u8 = (UART + 0x05) as *mut u8;
pub const UART_LSR_EMPTY_MASK: u8 = 0x40;

pub fn lib_putc(ch: u8) -> u8 {
    while unsafe { *UART_LSR & UART_LSR_EMPTY_MASK } == 0 {}
    unsafe { *UART_THR = ch };
    ch
}

pub fn lib_puts(s: &[u8]) {
    for &ch in s {
        lib_putc(ch);
    }
}

#[no_mangle]
pub extern "C" fn os_main() -> ! {
    lib_puts(HELLO);
    loop {}
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
