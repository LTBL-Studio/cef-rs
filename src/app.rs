use std::ptr::null_mut;

use cef_sys::{
    cef_app_t, cef_browser_process_handler_t, cef_command_line_t, cef_do_message_loop_work,
    cef_execute_process, cef_get_exit_code, cef_initialize, cef_quit_message_loop,
    cef_render_process_handler_t, cef_run_message_loop, cef_shutdown, cef_string_t,
};

use crate::{
    args::Args, browser_process_handler::BrowserProcessHandler, command_line::CommandLine,
    rc::RcImpl, render_process_handler::RenderProcessHandler, settings::Settings,
    string::CefString, ResultCode,
};

/// See [cef_app_t] for more documentation.
pub trait App: Sized {
    type RenderProcessHandler: RenderProcessHandler;
    type BrowserProcessHandler: BrowserProcessHandler;

    fn on_before_command_line_processing(
        &self,
        _process_type: Option<CefString>,
        _command_line: CommandLine,
    ) {
    }
    
    fn get_render_process_handler(
        &self
    ) -> Option<&Self::RenderProcessHandler> {
        None
    }
    
    fn get_browser_process_handler(
        &self
    ) -> Option<&Self::BrowserProcessHandler> {
        None
    }

    fn into_raw(self) -> *mut cef_app_t {
        let mut object: cef_app_t = unsafe { std::mem::zeroed() };

        object.on_before_command_line_processing = Some(on_before_command_line_processing::<Self>);
        object.get_render_process_handler = Some(get_render_process_handler::<Self>);
        object.get_browser_process_handler = Some(get_browser_process_handler::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

extern "C" fn on_before_command_line_processing<I: App>(
    this: *mut cef_app_t,
    process_type: *const cef_string_t,
    command_line: *mut cef_command_line_t,
) {
    let app: &mut RcImpl<_, I> = RcImpl::get(this);
    let process_type = unsafe { CefString::from_raw(process_type).ok() };
    let cmd = unsafe { CommandLine::from_raw(command_line) };

    app.interface
        .on_before_command_line_processing(process_type, cmd);
}

extern "C" fn get_render_process_handler<I: App>(this: *mut cef_app_t) -> *mut cef_render_process_handler_t {
    let app: &mut RcImpl<_, I> = RcImpl::get(this);
    let res = app.interface.get_render_process_handler();

    match res {
        Some(handler) => handler.get_raw(),
        None => null_mut(),
    }
}

extern "C" fn get_browser_process_handler<I: App>(this: *mut cef_app_t) -> *mut cef_browser_process_handler_t {
    let app: &mut RcImpl<_, I> = RcImpl::get(this);
    let res = app.interface.get_browser_process_handler();

    match res {
        Some(handler) => handler.get_raw(),
        None => null_mut(),
    }
}

/// See [cef_execute_process] for more documentation.
pub fn execute_process<T: App>(args: &Args, app: Option<T>) -> i32 {
    let args = args.to_raw();
    let app = app
        .map(|app| app.into_raw())
        .unwrap_or(std::ptr::null_mut());

    unsafe { cef_execute_process(&args, app, std::ptr::null_mut()) }
}

/// See [cef_initialize] for more documentation.
pub fn initialize<T: App>(
    args: &Args,
    settings: &Settings,
    app: Option<T>,
) -> Result<(), ResultCode> {
    let args = args.to_raw();
    let settings = settings.get_raw();
    let app = app
        .map(|app| app.into_raw())
        .unwrap_or(std::ptr::null_mut());

    if unsafe { cef_initialize(&args, &settings, app, std::ptr::null_mut()) } == 1 {
        Ok(())
    } else {
        Err(
            crate::utils::integer_to_error_code(unsafe { cef_get_exit_code() })
                .expect("Exit code is not a known result code"),
        )
    }
}

/// See [cef_run_message_loop] for more documentation.
pub fn run_message_loop() {
    unsafe { cef_run_message_loop() }
}

/// See [cef_do_message_loop_work] for more documentation.
pub fn do_message_loop_work() {
    unsafe { cef_do_message_loop_work() }
}

/// See [cef_quit_message_loop] for more documentation.
pub fn quit_message_loop() {
    unsafe { cef_quit_message_loop() }
}

/// See [cef_shutdown] for more documentation.
pub fn shutdown() {
    unsafe { cef_shutdown() }
}