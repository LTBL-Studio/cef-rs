use std::time::Duration;

use cef_sys::{
    cef_currently_on, cef_post_delayed_task, cef_post_task, cef_task_runner_get_for_current_thread, cef_task_runner_get_for_thread, cef_task_t, cef_thread_id_t
};

use crate::{rc::RcImpl, task_runner::TaskRunner};

pub type ThreadId = cef_thread_id_t;

/// See [cef_task_t] for more documentation.
pub trait Task: Sized {
    fn execute(self);

    fn into_raw(self) -> *mut cef_task_t {
        let mut object: cef_task_t = unsafe { std::mem::zeroed() };

        object.execute = Some(execute::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

/// See [cef_currently_on] for more documentation.
pub fn currently_on(thread_id: ThreadId) -> bool {
    unsafe { cef_currently_on(thread_id) > 0 }
}

/// See [cef_post_task] for more documentation.
pub fn post_task(thread_id: ThreadId, task: impl Task) -> bool {
    unsafe { cef_post_task(thread_id, task.into_raw()) > 0 }
}

/// See [cef_post_delayed_task] for more documentation.
pub fn post_delayed_task(thread_id: ThreadId, task: impl Task, delay: Duration) -> bool {
    unsafe { cef_post_delayed_task(thread_id, task.into_raw(), delay.as_millis() as i64) > 0 }
}

/// See [cef_task_runner_get_for_current_thread] for more documentation.
pub fn task_runner_get_for_current_thread() -> TaskRunner {
    unsafe { TaskRunner::from_raw(cef_task_runner_get_for_current_thread()) }
}

/// See [cef_task_runner_get_for_thread] for more documentation.
pub fn task_runner_get_for_thread(thread_id: ThreadId) -> TaskRunner {
    unsafe { TaskRunner::from_raw(cef_task_runner_get_for_thread(thread_id)) }
}

extern "C" fn execute<T: Task>(this: *mut cef_task_t) {
    let obj = unsafe { Box::from_raw(this as *mut RcImpl<cef_task_t, T>) };
    obj.interface.execute();
}

impl<F: FnOnce()> Task for F {
    fn execute(self) {
        self()
    }
}
