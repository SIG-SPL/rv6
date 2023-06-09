use crate::context::TrapFrame;
use config::layout::{plic_pri, plic_sen, plic_spri, UART0, VIRTIO0};
use core::arch::global_asm;
use riscv::register::scause::{self, Exception, Interrupt, Trap};
use riscv::register::{sie, sstatus, stvec};

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
            #[cfg(feature = "graphics")]
            crate::io::virtio::gpu::flush().unwrap();
            // crate::sched::schedule();
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            // We currently only support UART interrupts.
            // TODO: Read the PLIC's claim/complete register to determine
            //   which device generated the interrupt.
            let mut ch = crate::sbi::console_getchar() as u8 as char;
            if ch == '\r' {
                ch = '\n';
            }
            // echo
            #[cfg(feature = "graphics")]
            {
                let mut tb = crate::io::graphics::TEXT_BUFFER.lock();
                tb.putc(ch);
            }
            #[cfg(not(feature = "graphics"))]
            {
                crate::sbi::console_putchar(ch as u8 as usize);
            }
            // push to stdin
            unsafe {
                crate::io::STDIN.push(ch);
            }
        }
        Trap::Exception(Exception::UserEnvCall) => {
            // This is crucial because SIE bit is cleared when exception occurs.
            // To receive ext intrs when handling syscalls, we need to set it again.
            intr_on();
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

pub fn init(hartid: usize) {
    extern "C" {
        fn __trap();
    }

    unsafe {
        stvec::write(__trap as usize, stvec::TrapMode::Direct);
        sie::set_sext();
        sie::set_stimer();
        // enable PLIC by setting desired IRQ priorities nonzero
        (plic_pri(UART0) as *mut u32).write_volatile(7);
        (plic_pri(VIRTIO0) as *mut u32).write_volatile(1);
        // set enable bits
        (plic_sen(hartid) as *mut u32).write_volatile((1 << UART0) | (1 << VIRTIO0));
        // set this hart's S-mode priority threshold to 0
        (plic_spri(hartid) as *mut u32).write_volatile(0);
        // enable uart serial
        (0x10000001 as *mut u8).write_volatile(1);
        sstatus::set_sie();
        timer::set_next_trigger();
    }
}

/// TODO: use IO trait to implement this
mod timer {
    use riscv::register::time;
    pub fn get_time() -> usize {
        time::read()
    }

    // pub fn get_time_ms() -> usize {
    //     get_time() / (CLOCK_FREQ / MSEC_PER_TICK)
    // }

    pub const CLOCK_FREQ: usize = 12500000;
    pub const TICKS_PER_SEC: usize = 100;
    // pub const MSEC_PER_TICK: usize = 1000;

    pub fn set_next_trigger() {
        let time = get_time();
        crate::sbi::set_timer(time + CLOCK_FREQ / TICKS_PER_SEC);
    }
}
