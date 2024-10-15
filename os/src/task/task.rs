//! Types related to task management

use crate::syscall::process::TaskInfo;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// task infos
    pub task_infos: TaskInfo,
    /// The task context
    pub task_cx: TaskContext,
    /// user time in us
    pub user_time: usize,
    /// kernel time in us
    pub kernel_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
