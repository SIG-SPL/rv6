extern crate alloc;

use alloc::vec::Vec;

use crate::context::{Context, TrapFrame};
use crate::memlayout::{kstack, STACKSIZE};
use crate::sync::SpinLock;
use config::syscall::*;
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
    for _ in 0..10000000 {
        unsafe {
            asm!("nop");
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
    unreachable!()
}

#[no_mangle]
pub fn forkret() -> ! {
    unsafe {
        riscv::register::sepc::write(loop_print as usize);
        asm!("sret")
    }
    unreachable!()
}

pub fn init() -> ! {
    let mut tm = TASK_MANAGER.lock();
    tm.init();
    drop(tm);
    unsafe {
        asm!("mv {}, sp", in(reg) kstack(0) + STACKSIZE);
        riscv::register::sepc::write(loop_print as usize);
        asm!("sret")
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
        info!("Task {} created", pid);
        &mut self.tasks[pid]
    }

    pub fn switch_task(&mut self) -> (usize, usize) {
        let current_pid = self.current_pid;
        let next_pid = (current_pid + 1) % self.tasks.len();
        let ctx1: usize;
        let ctx2: usize;
        {
            let next_task = &mut self.tasks[next_pid];
            next_task.set_state(TaskState::Running);
            ctx1 = &next_task.context as *const Context as usize;
        }
        {
            let current_task = &mut self.tasks[current_pid];
            current_task.set_state(TaskState::Ready);
            ctx2 = &current_task.context as *const Context as usize;
        }
        self.current_pid = next_pid;
        (ctx2, ctx1)
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
pub struct Task {
    pub pid:            usize,
    pub state:          TaskState,
    pub kstack:         usize,
    pub context:        Context,
    pub trapframe:      TrapFrame,
}

impl Task {
    pub fn new(pid: usize) -> Self {
        let mut task = Self {
            pid,
            state: TaskState::Ready,
            kstack: kstack(pid),
            context: Context::default(),
            trapframe: TrapFrame::default(),
        };
        task.context.sp = task.kstack;
        task.context.ra = forkret as usize;
        task
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }
}
