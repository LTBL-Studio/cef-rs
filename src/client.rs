use std::{ptr::null_mut, sync::Arc};

use cef_sys::{cef_browser_t, cef_client_t, cef_frame_t, cef_life_span_handler_t, cef_load_handler_t, cef_process_id_t, cef_process_message_t, cef_render_handler_t};

use crate::{frame::Frame, life_span_handler::LifeSpanHandler, load_handler::LoadHandler, process::{ProcessId, ProcessMessage}, rc::RcImpl, render_handler::RenderHandler, Browser};

/// See [cef_client_t] for more documentation.
pub trait Client: Sized {
    type RenderHandler: RenderHandler;
    type LoadHandler: LoadHandler;
    type LifeSpanHandler: LifeSpanHandler;

    fn get_render_handler(&self) -> Option<&Self::RenderHandler> {
        None
    }

    fn get_load_handler(&self) -> Option<&Self::LoadHandler> {
        None
    }

    fn get_life_span_handler(&self) -> Option<&Self::LifeSpanHandler> {
        None
    }

    fn on_process_message_received(
        &self,
        _browser: Browser,
        _frame: Frame,
        _source_process: ProcessId,
        _message: ProcessMessage,
    ) -> bool {
        false
    }

    // TODO draw the rest owl
    fn into_raw(self) -> *mut cef_client_t {
        let mut object: cef_client_t = unsafe { std::mem::zeroed() };

        object.get_render_handler = Some(get_render_handler::<Self>);
        object.get_load_handler = Some(get_load_handler::<Self>);
        object.get_life_span_handler = Some(get_life_span_handler::<Self>);
        object.on_process_message_received = Some(on_process_message_received::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

extern "C" fn get_render_handler<C: Client>(this: *mut cef_client_t) -> *mut cef_render_handler_t {
    let client: &mut RcImpl<_, C> = RcImpl::get(this);
    let res = client.interface.get_render_handler();

    match res {
        Some(render_handler) => render_handler.get_raw(),
        None => null_mut(),
    }
}

extern "C" fn get_load_handler<C: Client>(this: *mut cef_client_t) -> *mut cef_load_handler_t {
    let client: &mut RcImpl<_, C> = RcImpl::get(this);
    let res = client.interface.get_load_handler();

    match res {
        Some(load_handler) => load_handler.get_raw(),
        None => null_mut(),
    }
}

extern "C" fn get_life_span_handler<C: Client>(this: *mut cef_client_t) -> *mut cef_life_span_handler_t {
    let client: &mut RcImpl<_, C> = RcImpl::get(this);
    let res = client.interface.get_life_span_handler();

    match res {
        Some(life_span_handler) => life_span_handler.get_raw(),
        None => null_mut(),
    }
}

extern "C" fn on_process_message_received<C: Client>(
    this: *mut cef_client_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    source_process: cef_process_id_t,
    message: *mut cef_process_message_t,
) -> ::std::os::raw::c_int {
    let handler: &mut RcImpl<_, C> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let frame = unsafe { Frame::from_raw(frame) };
    let message = unsafe { ProcessMessage::from_raw(message) };

    handler
        .interface
        .on_process_message_received(browser, frame, source_process, message).into()
}