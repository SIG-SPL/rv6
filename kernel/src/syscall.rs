#![allow(dead_code)]
use crate::TrapFrame;

// syscall number
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_OPEN: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_SLEEP: usize = 101;
pub const SYSCALL_SBARK: usize = 400;
// syscall register index
pub const SYSCALL_REG_NUM: usize = 17; // a7
pub const SYSCALL_REG_ARG0: usize = 10; // a0
pub const SYSCALL_REG_ARG1: usize = 11;
pub const SYSCALL_REG_ARG2: usize = 12;
pub const SYSCALL_REG_ARG3: usize = 13;
pub const SYSCALL_REG_ARG4: usize = 14;
pub const SYSCALL_REG_ARG5: usize = 15;
pub const SYSCALL_REG_ARG6: usize = 16;
pub const SYSCALL_REG_RET: usize = 10;
// STDIN/STDOUT/STDERR
pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;
pub const STDERR: usize = 2;

pub fn do_syscall(context: &mut TrapFrame) {
    match context.regs[SYSCALL_REG_NUM] {
        SYSCALL_EXIT => {
            println!("exit code: {}", context.regs[SYSCALL_REG_RET]);
            // schedule();
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
                        print!(
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
        _ => {
            println!("unknown syscall");
        }
    }
}
