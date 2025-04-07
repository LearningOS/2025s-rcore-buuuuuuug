//! Types related to task management

use super::TaskContext;

use crate::syscall;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in its lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// 记录当前任务每个系统调用触发的次数，如果系统调用增加，那么这个位置的数组大小需要对应增加
    pub sys_call_cnt: [u8; syscall::ALL_SYSCALL_CNT],
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
