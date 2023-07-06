use crate::TrapFrame;

use config::std_io::*;
use config::syscall::*;

pub fn do_syscall(context: &mut TrapFrame) {
    match context.regs[SYSCALL_REG_NUM] {
        SYSCALL_EXIT => {
            let pm = crate::proc::PROC_MANAGER.lock();
            debug!(
                "Task {} exited with code: {}",
                pm.current_pid, context.regs[SYSCALL_REG_RET]
            );
            drop(pm);
            crate::sched::schedule();
        }
        SYSCALL_GETPID => {
            let pm = crate::proc::PROC_MANAGER.lock();
            context.regs[SYSCALL_REG_RET] = pm.current_pid;
        }
        SYSCALL_WRITE => {
            let fd = context.regs[SYSCALL_REG_ARG0];
            let buf = context.regs[SYSCALL_REG_ARG1] as *const u8;
            let len = context.regs[SYSCALL_REG_ARG2];
            let p = buf;
            unsafe {
                match fd {
                    STDOUT | STDERR => {
                        use core::fmt::Write;
                        crate::stdio::Stdout
                            .write_str(core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                                p, len,
                            )))
                            .unwrap();
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
            match fd {
                STDIN => {
                    let mut cnt = 0;
                    for i in 0..len {
                        let ch = unsafe { crate::stdio::STDIN.pop() };
                        match ch {
                            '\r' | '\n' => {
                                if cnt > 0 {
                                    break;
                                }
                            },
                            _ => unsafe {
                                *buf.add(i) = ch as u8;
                                cnt += 1;
                            },
                        }
                    }
                    context.regs[SYSCALL_REG_RET] = cnt;
                }
                _ => todo!("only support stdin, which is fd=0, but got fd={}", fd),
            }
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
