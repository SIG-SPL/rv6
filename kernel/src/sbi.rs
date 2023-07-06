//! src/sbi.rs
#![allow(dead_code)]

use config::sbi::*;
use core::arch::asm;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    let ch = match c as u8 as char {
        '\r' => '\n',
        _ => c as u8 as char,
    } as usize;
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call(SRST_EXTENSION, SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!")
}

pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, 0, time, 0, 0);
}

pub fn get_timer() -> usize {
    riscv::register::time::read()
}
