use crate::TrapFrame;

use crate::sched::schedule;
use config::std_io::*;
use config::syscall::*;

struct DispOut;

impl core::fmt::Write for DispOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut tb = crate::graphics::TEXT_BUFFER.lock();
        for c in s.chars() {
            tb.putc(c);
        }
        Ok(())
    }
}

macro_rules! uprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        $crate::syscall::DispOut.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

pub fn do_syscall(context: &mut TrapFrame) {
    match context.regs[SYSCALL_REG_NUM] {
        SYSCALL_EXIT => {
            let pm = crate::proc::PROC_MANAGER.lock();
            debug!("Task {} exited with code: {}", pm.current_pid, context.regs[SYSCALL_REG_RET]);
            drop(pm);
            schedule();
        }
        SYSCALL_GETPID => {
            let pm = crate::proc::PROC_MANAGER.lock();
            context.regs[SYSCALL_REG_RET] = pm.current_pid;
        }
        SYSCALL_WRITE => {
            let fd = context.regs[SYSCALL_REG_ARG0];
            let buf = context.regs[SYSCALL_REG_ARG1] as *const u8;
            let len = context.regs[SYSCALL_REG_ARG2];
            debug!("write: fd={}, buf={:p}, len={}", fd, buf, len);
            let p = buf;
            unsafe {
                match fd {
                    STDOUT | STDERR => {
                        uprint!(
                            "{}",
                            core::str::from_utf8_unchecked(core::slice::from_raw_parts(p, len))
                        );
                    }
                    _ => todo!(
                        "only support stdout/stderr, which is fd=1/2, but got fd={}",
                        fd
                    ),
                }
            }
            context.regs[SYSCALL_REG_RET] = len;
        }
        SYSCALL_READ => {
            let fd = context.regs[SYSCALL_REG_ARG0];
            let buf = context.regs[SYSCALL_REG_ARG1] as *mut u8;
            let len = context.regs[SYSCALL_REG_ARG2];
            todo!("read from fd={}, buf={:p}, len={}", fd, buf, len);
        }
        SYSCALL_SLEEP => {
            let time = context.regs[SYSCALL_REG_ARG0];
            // add to timer list
            todo!("sleep for {} ticks", time);
        }
        SYSCALL_GETTIME => {
            context.regs[SYSCALL_REG_RET] = crate::sbi::get_timer();
        }
        _ => {
            panic!("unknown syscall number {}", context.regs[SYSCALL_REG_NUM]);
        }
    }
}
