use crate::task::TASK_MANAGER;

pub fn schedule() {
    extern "C" {
        fn swtch(old: usize, new: usize);
    }

    println!("Scheduling");
    let mut tm = TASK_MANAGER.lock();
    let (old, new) = tm.switch_task();
    drop(tm);
    unsafe {
        swtch(old, new);
    }
}
