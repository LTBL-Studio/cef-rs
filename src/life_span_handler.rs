use crate::{
    rc::RcImpl, Browser,
};
use cef_sys::{
    cef_browser_t,
    cef_life_span_handler_t,
};

/// See [cef_load_handler_t] for more documentation.
pub trait LifeSpanHandler: Sized {
    // type Client: Client;
    // fn on_before_dev_tools_popup(
    //     &self,
    //     _browser: Browser,
    //     _window_info: WindowInfo,
    //     _client: &impl Client,
    //     _settings: BrowserSettings,
    //     _extra_info: &mut DictionaryValue,
    //     _use_default_window: bool,
    // ) {
    // }
    fn on_after_created(&self, _browser: Browser) {}
    fn do_close(&self, _browser: Browser) -> bool {
        false
    }
    fn on_before_close(&self, _browser: Browser) {}

    fn get_raw(&self) -> *mut cef_life_span_handler_t {
        let mut object: cef_life_span_handler_t = unsafe { std::mem::zeroed() };

        // object.on_before_dev_tools_popup = Some(on_before_dev_tools_popup::<Self>);
        object.on_after_created = Some(on_after_created::<Self>);
        object.do_close = Some(do_close::<Self>);
        object.on_before_close = Some(on_before_close::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl LifeSpanHandler for () {
    fn on_after_created(&self, _browser: Browser) {}
    fn on_before_close(&self, _browser: Browser) {}
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
//     let handler: &mut RcImpl<_, &L> = RcImpl::get(this);
//     let browser = unsafe { Browser::from_raw(browser) };
//     let window_info = unsafe { WindowInfo::from_mut_ptr(window_info) };
//     let client: &mut RcImpl<*mut cef_sys::_cef_client_t, L::Client> = RcImpl::get(client);
//     let settings = BrowserSettings::from_mut_ptr(settings);
//     let mut extra_info = unsafe { DictionaryValue::from_raw(*extra_info) };
//     handler.interface.on_before_dev_tools_popup(
//         browser,
//         window_info,
//         &client.interface,
//         settings,
//         &mut extra_info,
//         unsafe { *use_default_window } > 0,
//     );
// }

extern "C" fn on_after_created<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) {
    let handler: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.on_after_created(browser);
}

extern "C" fn do_close<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) -> ::std::os::raw::c_int {
    let handler: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.do_close(browser).into()
}

extern "C" fn on_before_close<L: LifeSpanHandler>(
    this: *mut cef_life_span_handler_t,
    browser: *mut cef_browser_t,
) {
    let handler: &mut RcImpl<_, &L> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    handler.interface.on_before_close(browser);
}
