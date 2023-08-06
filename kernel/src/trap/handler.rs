use core::arch::global_asm;
use riscv::register::scause::{self, Exception, Interrupt, Trap};

use crate::context::TrapFrame;
use crate::trap::plic::{self, ExternalInterrupt};

global_asm!(include_str!("../asm/trap.S"));

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
            crate::trap::timer::set_next_trigger();
            #[cfg(feature = "graphics")]
            crate::io::virtio::gpu::flush().unwrap();
            // crate::sched::schedule();
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            let intr = plic::next();
            match intr {
                ExternalInterrupt::None => panic!("unexpected external interrupt"),
                ExternalInterrupt::UART => {
                    let mut ch = crate::sbi::console_getchar() as u8 as char;
                    if ch == '\r' {
                        ch = '\n';
                    }
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
                _ => warn!("unimplemented external interrupt {:?}", intr),
            }
            intr.complete()
        }
        Trap::Exception(Exception::UserEnvCall) => {
            // This is crucial because SIE bit is cleared when exception occurs.
            // To receive ext intrs when handling syscalls, we need to set it again.
            crate::trap::intr_on();
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
