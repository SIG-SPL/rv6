use crate::context::TrapFrame;
use core::arch::global_asm;
use riscv::register::scause::{self, Exception, Interrupt, Trap};
use riscv::register::{sie, stvec};

global_asm!(include_str!("asm/trap.S"));

#[no_mangle]
pub fn trap_handler(ctx: &mut TrapFrame) -> &mut TrapFrame {
    let scause = scause::read();
    assert_eq!(
        scause.bits(),
        ctx.scause,
        "scause not equal before and after interrupt"
    );
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            timer::set_next_trigger();
            // crate::sched::schedule();
        }
        Trap::Exception(Exception::UserEnvCall) => {
            crate::syscall::do_syscall(ctx);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            panic!("IllegalInstruction")
        }
        _ => panic!("unhandled trap {:?}\n{:#x?}", scause.cause(), ctx),
    }
    if scause.is_exception() {
        ctx.sepc += 4;
    }
    ctx
}

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

pub fn init() {
    extern "C" {
        fn __trap();
    }

    unsafe {
        stvec::write(__trap as usize, stvec::TrapMode::Direct);
        sie::set_stimer();
        timer::set_next_trigger();
    }
}

/// TODO: use IO trait to implement this
mod timer {
    use riscv::register::time;
    pub fn get_time() -> usize {
        time::read()
    }

    pub fn get_time_ms() -> usize {
        get_time() / (CLOCK_FREQ / MSEC_PER_TICK)
    }

    pub const CLOCK_FREQ: usize = 12500000;
    pub const TICKS_PER_SEC: usize = 100;
    pub const MSEC_PER_TICK: usize = 1000;

    pub fn set_next_trigger() {
        let time = get_time();
        crate::sbi::set_timer(time + CLOCK_FREQ / TICKS_PER_SEC);
    }
}
