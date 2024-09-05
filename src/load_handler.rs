use cef_sys::{
    cef_browser_t, cef_errorcode_t, cef_frame_t, cef_load_handler_t, cef_string_t,
    cef_transition_type_t,
};

use crate::{frame::Frame, rc::RcImpl, string::CefString, Browser, ErrorCode, TransitionType};

/// See [cef_load_handler_t] for more documentation.
pub trait LoadHandler: Sized {
    //get_accessibility_handler

    /// Called when the loading state has changed.
    /// 
    /// This callback will be executed twice -- once when loading is initiated either programmatically or by user action, and once when loading is terminated due to completion, cancellation of failure.
    /// 
    /// It will be called before any calls to on_load_start and after all calls to on_load_error and/or on_load_end.
    fn on_loading_state_change(
        &self,
        browser: &Browser,
        is_loading: bool,
        can_go_back: bool,
        can_go_forward: bool,
    );

    /// Called after a navigation has been committed and before the browser begins loading contents in the frame.
    ///
    /// Call the `is_main()` function to check if this frame is the main frame.
    /// 
    /// `transition_type` provides information about the source of the navigation and an accurate value is only available in the browser process.
    /// 
    /// Multiple frames may be loading at the same time. Sub-frames may start or continue loading after the main frame load has ended.
    /// This function will not be called for same page navigations (fragments, history state, etc.) or for navigations that fail or are canceled before commit.
    /// 
    /// For notification of overall browser load status use on_loading_state_change instead.
    fn on_load_start(
        &self,
        browser: &Browser,
        frame: &mut Frame,
        transition_type: TransitionType,
    );

    /// Called when the browser is done loading a frame.
    /// 
    /// Call the `is_main()` function to check if this frame is the main frame.
    /// 
    /// Multiple frames may be loading at the same time. Sub-frames may start or continue loading after the main frame load has ended.
    /// This function will not be called for same page navigations (fragments, history state, etc.) or for navigations that fail or are canceled before commit.
    /// 
    /// For notification of overall browser load status use on_loading_state_change instead.
    fn on_load_end(&self, browser: &Browser, frame: &mut Frame, http_status_code: i32);

    /// Called when a navigation fails or is canceled.
    /// 
    /// This function may be called by itself if before commit or in combination with on_load_start/on_load_end if after commit.
    /// 
    /// - `errorCode` is the error code number
    /// - `errorText` is the error text
    /// - `failedUrl` is the URL that failed to load
    fn on_load_error(
        &self,
        browser: &Browser,
        frame: &mut Frame,
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
        _frame: &mut Frame,
        _transition_type: TransitionType,
    ) {
    }

    fn on_load_end(&self, _browser: &Browser, _frame: &mut Frame, _http_status_code: i32) {}

    fn on_load_error(
        &self,
        _browser: &Browser,
        _frame: &mut Frame,
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
    let client: &mut RcImpl<_, &L> = RcImpl::get(this);
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
    let client: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let mut frame = unsafe { Frame::from_raw(frame) };
    client
        .interface
        .on_load_start(&browser, &mut frame, transition_type);
}

extern "C" fn on_load_end<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    http_status_code: i32,
) {
    let client: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let mut frame = unsafe { Frame::from_raw(frame) };
    client
        .interface
        .on_load_end(&browser, &mut frame, http_status_code);
}

extern "C" fn on_load_error<L: LoadHandler>(
    this: *mut cef_load_handler_t,
    browser: *mut cef_browser_t,
    frame: *mut cef_frame_t,
    error_code: cef_errorcode_t,
    error_text: *const cef_string_t,
    failed_url: *const cef_string_t,
) {
    let client: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let mut frame = unsafe { Frame::from_raw(frame) };
    client.interface.on_load_error(
        &browser,
        &mut frame,
        error_code,
        unsafe { CefString::from_raw(error_text).unwrap_or(CefString::new("unknown error")) },
        unsafe {
            CefString::from_raw(failed_url).unwrap_or(CefString::new("url conversion failed"))
        },
    );
}
