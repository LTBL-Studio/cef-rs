use cef_sys::{cef_currently_on, cef_post_task, cef_task_t, cef_thread_id_t};

use crate::rc::RcImpl;

pub type ThreadId = cef_thread_id_t;

/// See [cef_task_t] for more documentation.
pub trait Task: Sized {
    fn execute(
        &self
    ) {
    }

    fn into_raw(self) -> *mut cef_task_t {
        let mut object: cef_task_t = unsafe { std::mem::zeroed() };

        object.execute = Some(execute::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}


/// See [cef_currently_on] for more documentation.
pub fn currently_on(thread_id: ThreadId) -> bool{
    unsafe { cef_currently_on(thread_id) > 0 }
}

/// See [cef_post_task] for more documentation.
pub fn post_task(thread_id: ThreadId, task: impl Task) -> bool{
    unsafe { cef_post_task(thread_id, task.into_raw()) > 0 } 
}

extern "C" fn execute<T: Task>(
    this: *mut cef_task_t
) {
    let obj: &mut RcImpl<_, T> = RcImpl::get(this);

    obj.interface
        .execute();
}
