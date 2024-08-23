use cef_sys::{
    cef_browser_t, cef_errorcode_t, cef_frame_t, cef_load_handler_t, cef_string_t,
    cef_transition_type_t,
};

use crate::{rc::RcImpl, string::CefString, Browser, ErrorCode, TransitionType};

/// See [cef_load_handler_t] for more documentation.
pub trait LoadHandler: Sized {
    //get_accessibility_handler
    fn on_loading_state_change(
        &self,
        browser: &Browser,
        is_loading: bool,
        can_go_back: bool,
        can_go_forward: bool,
    );
    fn on_load_start(
        &self,
        browser: &Browser,
        frame: *mut cef_frame_t,
        transition_type: TransitionType,
    );
    fn on_load_end(&self, browser: &Browser, frame: *mut cef_frame_t, http_status_code: i32);
    fn on_load_error(
        &self,
        browser: &Browser,
        frame: *mut cef_frame_t,
        error_code: ErrorCode,
        error_text: CefString,
        failed_url: CefString,
    );

    fn get_raw(&self) -> *mut cef_load_handler_t {
        let mut object: cef_load_handler_t = unsafe { std::mem::zeroed() };

        object.on_loading_state_change = Some(on_loading_state_change::<Self>);
        object.on_load_start = Some(on_load_start::<Self>);
        object.on_load_end = Some(on_load_end::<Self>);
        object.on_load_error = Some(on_load_error::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl LoadHandler for () {
    fn on_loading_state_change(
        &self,
        _browser: &Browser,
        _is_loading: bool,
        _can_go_back: bool,
        _can_go_forward: bool,
    ) {
    }

    fn on_load_start(
        &self,
        _browser: &Browser,
        _frame: *mut cef_frame_t,
        _transition_type: TransitionType,
    ) {
    }

    fn on_load_end(&self, _browser: &Browser, _frame: *mut cef_frame_t, _http_status_code: i32) {}

    fn on_load_error(
        &self,
        _browser: &Browser,
        _frame: *mut cef_frame_t,
        _error_code: ErrorCode,
        _error_text: CefString,
        _failed_url: CefString,
    ) {
    }
}

extern "C" fn on_loading_state_change<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    is_loading: ::std::os::raw::c_int,
    can_go_back: ::std::os::raw::c_int,
    can_go_forward: ::std::os::raw::c_int,
) {
    let client: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client.interface.on_loading_state_change(
        &browser,
        is_loading > 0,
        can_go_back > 0,
        can_go_forward > 0,
    );
}

extern "C" fn on_load_start<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    transition_type: cef_transition_type_t,
) {
    let client: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client
        .interface
        .on_load_start(&browser, frame, transition_type);
}

extern "C" fn on_load_end<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    http_status_code: i32,
) {
    let client: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client
        .interface
        .on_load_end(&browser, frame, http_status_code);
}

extern "C" fn on_load_error<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    error_code: cef_errorcode_t,
    error_text: *const cef_string_t,
    failed_url: *const cef_string_t,
) {
    let client: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client.interface.on_load_error(
        &browser,
        frame,
        error_code,
        unsafe { CefString::from_raw(error_text).unwrap_or(CefString::new("unknown error")) },
        unsafe {
            CefString::from_raw(failed_url).unwrap_or(CefString::new("url conversion failed"))
        },
    );
}
