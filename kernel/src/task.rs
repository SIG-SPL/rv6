extern crate alloc;

use crate::context::{Context, TrapFrame};
use crate::sync::SpinLock;
use alloc::vec::Vec;
use core::arch::asm;

lazy_static! {
    pub static ref TASK_MANAGER: SpinLock<TaskManager> =
        SpinLock::new(TaskManager::new(), "TaskManagerLock");
}

pub struct TaskManager {
    pub tasks: Vec<Task>,
    pub current_pid: usize,
}

#[no_mangle]
pub fn loop_print() -> ! {
    use config::syscall::*;
    loop {
        let pid: usize;
        unsafe {
            asm!(
                "ecall",
                out("a0") pid,
                in("a7") SYSCALL_GETPID,
            );
        }
        let string = alloc::format!("Hello from task {}\n", pid);
        unsafe {
            asm!(
                "ecall",
                inlateout("a0") 1 => _,
                in("a1") string.as_ptr(),
                in("a2") string.len(),
                in("a7") SYSCALL_WRITE,
            );
        }
        for _ in 0..10000 {
            for _ in 0..100000 {
                unsafe {
                    asm!("nop");
                }
            }
        }
        // exit
        unsafe {
            asm!(
                "ecall",
                inlateout("a0") 0 => _,
                in("a7") SYSCALL_EXIT,
            );
        }
    }
}

#[no_mangle]
pub fn forkret() -> ! {
    unsafe {
        riscv::register::sepc::write(loop_print as usize);
        asm!("sret")
    }
    unreachable!()
}

/// Spawn task 0
/// - We don't need to & must not store any context here
/// as the current context belongs to the kernel main thread.
/// - The kernel thread should never involve in context switching.
pub fn init() -> ! {
    let mut tm = TASK_MANAGER.lock();
    tm.init();
    let sp = tm.tasks[0].context.sp;
    let ra = tm.tasks[0].context.ra;
    drop(tm);
    unsafe {
        asm!(
            "mv sp, {}",
            "mv ra, {}",
            "ret",
            in(reg) sp,
            in(reg) ra
        );
    }
    unreachable!()
}

impl TaskManager {
    pub const fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_pid: 0,
        }
    }

    pub fn init(&mut self) {
        for _ in 0..5 {
            self.create_task();
        }
    }

    pub fn create_task(&mut self) -> &mut Task {
        let pid = self.tasks.len();
        let task = Task::new(pid);
        self.tasks.push(task);
        &mut self.tasks[pid]
    }

    pub fn switch_task(&mut self) -> (usize, usize) {
        let current_pid = self.current_pid;
        let next_pid = (current_pid + 1) % self.tasks.len();
        let ctx_new: usize;
        let ctx_old: usize;
        {
            let next_task = &mut self.tasks[next_pid];
            next_task.set_state(TaskState::Running);
            ctx_new = &next_task.context as *const Context as usize;
        }
        {
            let current_task = &mut self.tasks[current_pid];
            current_task.set_state(TaskState::Ready);
            ctx_old = &current_task.context as *const Context as usize;
        }
        self.current_pid = next_pid;
        (ctx_old, ctx_new)
    }
}

#[derive(Default)]
pub enum TaskState {
    Running,
    #[default]
    Ready,
    Blocked,
    Exited,
}

/// Task control block
/// Tasks run in user mode, but use kernel memory for now 
/// because we don't have virtual memory yet.
#[rustfmt::skip]
#[repr(align(4096))]
pub struct Task {
    /// process id
    pub pid:            usize,
    /// task state
    pub state:          TaskState,
    /// kernel stack
    pub kstack:         usize,
    pub context:        Context,
    pub trapframe:      TrapFrame,
}

impl Task {
    pub fn new(pid: usize) -> Self {
        let mut task = Self {
            pid,
            state: TaskState::default(),
            kstack: 0,
            context: Context::default(),
            trapframe: TrapFrame::default(),
        };
        task.kstack = &task as *const Task as usize + 4096;
        task.context.sp = task.kstack;
        task.context.ra = forkret as usize;
        task
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }
}
