use crate::{rc::RcImpl, Browser};
use cef_sys::{cef_browser_t, cef_life_span_handler_t};

/// See [cef_load_handler_t] for more documentation.
pub trait LifeSpanHandler: Sized {
    //get_accessibility_handler
    fn on_after_created(&self, browser: &Browser);
    fn do_close(&self, browser: &Browser) -> bool;
    fn on_before_close(&self, browser: &Browser);

    fn get_raw(&self) -> *mut cef_life_span_handler_t {
        let mut object: cef_life_span_handler_t = unsafe { std::mem::zeroed() };

        object.on_after_created = Some(on_after_created::<Self>);
        object.do_close = Some(do_close::<Self>);
        object.on_before_close = Some(on_before_close::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl LifeSpanHandler for () {
    fn on_after_created(&self, _browser: &Browser) {}
    fn do_close(&self, _browser: &Browser) -> bool {
        true
    }
    fn on_before_close(&self, _browser: &Browser) {}
}

// extern "C" fn on_before_dev_tools_popup<L: LifeSpanHandler>(
//     this: *mut cef_life_span_handler_t,
//     browser: *mut cef_browser_t,
//     window_info: *mut cef_window_info_t,
//     client: *mut *mut cef_client_t,
//     settings: *mut cef_browser_settings_t,
//     extra_info: *mut *mut cef_dictionary_value_t,
//     use_default_window: *mut ::std::os::raw::c_int,
// ) {
//     let handler: &mut RcImpl<_, L> = RcImpl::get(this);
//     let browser = unsafe { Browser::from_raw(browser) };
//     let window_info = unsafe { WindowInfo::from_mut_ptr(window_info) };
//     let client = RcImpl::get(client);
//     let settings = BrowserSettings::from_mut_ptr(settings);
//     handler.interface.on_before_dev_tools_popup(&browser, window_info, client.interface, settings, extra_info, unsafe { *use_default_window } > 0);
// }

extern "C" fn on_after_created<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) {
    let handler: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.on_after_created(&browser);
}

extern "C" fn do_close<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) -> ::std::os::raw::c_int {
    let handler: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.do_close(&browser).into()
}

extern "C" fn on_before_close<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) {
    let handler: &mut RcImpl<_, L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.on_before_close(&browser);
}
