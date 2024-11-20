use std::time::Duration;

use cef_sys::{cef_browser_process_handler_t, cef_command_line_t};

use crate::{rc::RcImpl, CommandLine};

/// See [cef_browser_process_handler_t] for more documentation.
pub trait BrowserProcessHandler: Sized {
    fn on_context_initialized(&self) {}
    fn on_before_child_process_launch(&self, _command_line: CommandLine) {}
    fn on_schedule_message_pump_work(&self, _delay: Duration) {}

    fn get_raw(&self) -> *mut cef_browser_process_handler_t {
        let mut object: cef_browser_process_handler_t = unsafe { std::mem::zeroed() };

        object.on_context_initialized = Some(on_context_initialized::<Self>);
        object.on_schedule_message_pump_work = Some(on_schedule_message_pump_work::<Self>);
        object.on_before_child_process_launch = Some(on_before_child_process_launch::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl BrowserProcessHandler for () {}

extern "C" fn on_context_initialized<H: BrowserProcessHandler>(
    this: *mut cef_browser_process_handler_t,
) {
    let handler: &mut RcImpl<_, &H> = RcImpl::get(this);
    handler.interface.on_context_initialized();
}

extern "C" fn on_before_child_process_launch<H: BrowserProcessHandler>(
    this: *mut cef_browser_process_handler_t,
    command_line: *mut cef_command_line_t,
) {
    let handler: &RcImpl<_, &H> = RcImpl::get(this);
    let cmd = unsafe { CommandLine::from_raw(command_line) };

    handler.interface.on_before_child_process_launch(cmd);
}

extern "C" fn on_schedule_message_pump_work<H: BrowserProcessHandler>(
    this: *mut cef_browser_process_handler_t,
    delay_ms: i64,
) {
    let handler: &mut RcImpl<_, &H> = RcImpl::get(this);

    handler
        .interface
        .on_schedule_message_pump_work(Duration::from_millis(delay_ms as u64));
}
