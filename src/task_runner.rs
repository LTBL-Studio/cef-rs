use std::time::Duration;

use cef_sys::cef_task_runner_t;

use crate::{thread::{Task, ThreadId}, wrapper};

wrapper!(
    #[doc = "See [cef_task_runner_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct TaskRunner(cef_task_runner_t);
);

impl TaskRunner {
    pub fn is_same(&self, task_runner: &TaskRunner) -> bool {
        self.0
            .is_same
            .map(|f| unsafe { f(self.0.get_raw(), task_runner.0.get_raw()) } > 0)
            .unwrap_or(true)
    }

    pub fn belongs_to_current_thread(&self) -> bool {
        self.0
            .belongs_to_current_thread
            .map(|f| unsafe { f(self.0.get_raw()) } > 0)
            .unwrap_or(true)
    }

    pub fn belongs_to_thread(&self, thread_id: ThreadId) -> bool {
        self.0
            .belongs_to_thread
            .map(|f| unsafe { f(self.0.get_raw(), thread_id) } > 0)
            .unwrap_or(true)
    }

    pub fn post_task(&self,  task: impl Task) -> bool {
        self.0
            .post_task
            .map(|f| unsafe { f(self.0.get_raw(), task.into_raw()) } > 0)
            .unwrap_or(true)
    }

    pub fn post_delayed_task(&self, task: impl Task, delay: Duration) -> bool {
        self.0
            .post_delayed_task
            .map(|f| unsafe { f(self.0.get_raw(), task.into_raw(), delay.as_millis() as i64) } > 0)
            .unwrap_or(true)
    }

}