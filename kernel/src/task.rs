use crate::sync::SpinLock;

lazy_static! {
    static ref TASK_MANAGER: SpinLock<TaskManager> = SpinLock::new(TaskManager::new());
}

pub struct TaskManager {

}

impl TaskManager {
    pub const fn new() -> Self {
        Self {

        }
    }
}

struct Task {

}