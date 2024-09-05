use std::ptr::null_mut;

use cef_sys::{
    cef_browser_t, cef_dictionary_value_t, cef_domnode_t, cef_frame_t, cef_load_handler_t, cef_process_id_t, cef_process_message_t, cef_render_process_handler_t, cef_v8context_t, cef_v8exception_t, cef_v8stack_trace_t
};

use crate::{
    dom::DomNode,
    frame::Frame,
    load_handler::LoadHandler,
    process::{ProcessId, ProcessMessage},
    rc::RcImpl,
    v8_context::V8Context,
    v8_exception::V8Exception,
    v8_stack_trace::V8StackTrace,
    values::DictionaryValue,
    Browser,
};

/// See [cef_render_process_handler_t] for more documentation.
pub trait RenderProcessHandler: Sized {
    type LoadHandler: LoadHandler;

    fn on_webkit_initialized(&self) {}
    fn on_browser_created(&self, _browser: Browser, _extra_info: DictionaryValue) {}
    fn on_browser_destroyed(&self, _browser: Browser) {}
    fn get_load_handler(&self) -> Option<&Self::LoadHandler> {
        None
    }
    fn on_context_created(&self, _browser: Browser, _frame: Frame, _context: V8Context) {}
    fn on_context_released(&self, _browser: Browser, _frame: Frame, _context: V8Context) {}
    fn on_uncaught_exception(
        &self,
        _browser: Browser,
        _frame: Frame,
        _context: V8Context,
        _exception: V8Exception,
        _stack_trace: V8StackTrace,
    ) {
    }
    fn on_focused_node_changed(&self, _browser: Browser, _frame: Frame, _node: DomNode) {}
    fn on_process_message_received(
        &self,
        _browser: Browser,
        _frame: Frame,
        _source_process: ProcessId,
        _message: ProcessMessage,
    ) -> bool {
        false
    }

    fn get_raw(&self) -> *mut cef_render_process_handler_t {
        let mut object: cef_render_process_handler_t = unsafe { std::mem::zeroed() };

        object.on_web_kit_initialized = Some(on_webkit_initialized::<Self>);
        object.on_browser_created = Some(on_browser_created::<Self>);
        object.on_browser_destroyed = Some(on_browser_destroyed::<Self>);
        object.get_load_handler = Some(get_load_handler::<Self>);
        object.on_context_created = Some(on_context_created::<Self>);
        object.on_context_released = Some(on_context_released::<Self>);
        object.on_uncaught_exception = Some(on_uncaught_exception::<Self>);
        object.on_focused_node_changed = Some(on_focused_node_changed::<Self>);
        object.on_process_message_received = Some(on_process_message_received::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl RenderProcessHandler for () {
    type LoadHandler = ();
}

extern "C" fn on_webkit_initialized<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    handler.interface.on_webkit_initialized();
}

extern "C" fn on_browser_created<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    extra_info: *mut cef_dictionary_value_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let extra_info = unsafe { DictionaryValue::from_raw(extra_info) };

    handler.interface.on_browser_created(browser, extra_info);
}

extern "C" fn on_browser_destroyed<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };

    handler.interface.on_browser_destroyed(browser);
}

extern "C" fn get_load_handler<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
) -> *mut cef_load_handler_t {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let res = handler.interface.get_load_handler();

    match res {
        Some(load_handler) => load_handler.get_raw(),
        None => null_mut(),
    }
}

extern "C" fn on_context_created<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    context: *mut cef_v8context_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let context = unsafe { V8Context::from_raw(context) };

    handler
        .interface
        .on_context_created(browser, frame, context);
}

extern "C" fn on_context_released<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    context: *mut cef_v8context_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let context = unsafe { V8Context::from_raw(context) };

    handler
        .interface
        .on_context_released(browser, frame, context);
}

extern "C" fn on_uncaught_exception<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    context: *mut cef_v8context_t,
    exception: *mut cef_v8exception_t,
    stack_trace: *mut cef_v8stack_trace_t,
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let context = unsafe { V8Context::from_raw(context) };
    let exception = unsafe { V8Exception::from_raw(exception) };
    let stack_trace = unsafe { V8StackTrace::from_raw(stack_trace) };

    handler
        .interface
        .on_uncaught_exception(browser, frame, context, exception, stack_trace);
}

extern "C" fn on_focused_node_changed<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    node: *mut cef_domnode_t
) {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let node = unsafe { DomNode::from_raw(node) };

    handler
        .interface
        .on_focused_node_changed(browser, frame, node);
}

extern "C" fn on_process_message_received<R: RenderProcessHandler>(
    this: *mut cef_render_process_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    source_process: cef_process_id_t,
    message: *mut cef_process_message_t,
) -> ::std::os::raw::c_int {
    let handler: &mut RcImpl<_, &R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let message = unsafe { ProcessMessage::from_raw(message) };

    handler
        .interface
        .on_process_message_received(browser, frame, source_process, message).into()
}