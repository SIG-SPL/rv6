use crate::proc::PROC_MANAGER;

pub fn schedule() {
    extern "C" {
        fn swtch(old: usize, new: usize);
    }

    let mut pm = PROC_MANAGER.lock();
    let (old, new) = pm.switch_task();
    drop(pm);
    unsafe {
        swtch(old, new);
    }
}
