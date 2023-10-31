//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// started
    pub started: bool,
    /// task start time
    pub start_time: usize,
    /// syscall times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}
/// impl default() for TaskControlBlock to simplify initialization
impl Default for TaskControlBlock {
    fn default() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            started: false,
            start_time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized 未初始化
    UnInit,
    /// ready to run 准备运行
    Ready,
    /// running 正在运行
    Running,
    /// exited  已退出
    Exited,
}
