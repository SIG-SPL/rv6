mod handler;
mod plic;
mod timer;

use riscv::register::{sie, stvec};

#[allow(dead_code)]
/// turn off interrupt
pub fn intr_off() {
    unsafe {
        riscv::register::sstatus::clear_sie();
    }
}

/// turn on interrupt
pub fn intr_on() {
    unsafe {
        riscv::register::sstatus::set_sie();
    }
}

pub fn init(hartid: usize) {
    extern "C" {
        fn __trap();
    }
    unsafe {
        stvec::write(__trap as usize, stvec::TrapMode::Direct);
        sie::set_sext();
        sie::set_stimer();
        plic::init(hartid);
        (0x10000001 as *mut u8).write_volatile(1);
        intr_on();
        timer::set_next_trigger();
    }
}
