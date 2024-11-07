use std::time::Duration;

use cef_sys::{
    cef_currently_on, cef_post_delayed_task, cef_post_task, cef_task_runner_get_for_current_thread,
    cef_task_runner_get_for_thread, cef_task_t, cef_thread_id_t,
};

use crate::{rc::RcImpl, task_runner::TaskRunner};

pub type ThreadId = cef_thread_id_t;

pub struct TaskWrapper<F> {
    func: Option<F>,
}

impl<F: FnOnce()> TaskWrapper<F> {
    pub fn new(func: F) -> Self {
        Self { func: Some(func) }
    }

    pub fn into_raw(self) -> *mut cef_task_t {
        let mut object: cef_task_t = unsafe { std::mem::zeroed() };

        object.execute = Some(Self::execute);

        RcImpl::new(object, self) as *mut _
    }

    extern "C" fn execute(task: *mut cef_task_t) {
        let task: &mut RcImpl<cef_task_t, Self> = RcImpl::get(task);

        if let Some(func) = task.interface.func.take() {
            (func)();
        }
    }
}

/// See [cef_currently_on] for more documentation.
pub fn currently_on(thread_id: ThreadId) -> bool {
    unsafe { cef_currently_on(thread_id) > 0 }
}

/// See [cef_post_task] for more documentation.
pub fn post_task(thread_id: ThreadId, task: impl FnOnce() + 'static) -> bool {
    unsafe { cef_post_task(thread_id, TaskWrapper::new(task).into_raw()) > 0 }
}

/// See [cef_post_delayed_task] for more documentation.
pub fn post_delayed_task(thread_id: ThreadId, task: impl FnOnce() + 'static, delay: Duration) -> bool {
    unsafe { cef_post_delayed_task(thread_id, TaskWrapper::new(task).into_raw(), delay.as_millis() as i64) > 0 }
}

/// See [cef_task_runner_get_for_current_thread] for more documentation.
pub fn task_runner_get_for_current_thread() -> Option<TaskRunner> {
    let p = unsafe { cef_task_runner_get_for_current_thread() };
    if p.is_null() {
        None
    } else {
        Some(unsafe { TaskRunner::from_raw(p) })
    }
}

/// See [cef_task_runner_get_for_thread] for more documentation.
pub fn task_runner_get_for_thread(thread_id: ThreadId) -> TaskRunner {
    unsafe { TaskRunner::from_raw(cef_task_runner_get_for_thread(thread_id)) }
}