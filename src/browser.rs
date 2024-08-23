use std::{ffi::c_int, ptr::null_mut};

use cef_sys::{
    cef_browser_host_create_browser, cef_browser_host_create_browser_sync, cef_browser_host_t,
    cef_browser_settings_t, cef_browser_t, cef_browser_view_create, cef_browser_view_t,
};

use crate::{
    client::Client, render_utils::PaintElementType, string::CefString, window::WindowInfo, wrapper, State, View
};

/// See [cef_browser_settings_t] for more documentation.
#[derive(Debug, Clone)]
pub struct BrowserSettings {
    pub windowless_frame_rate: usize,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: u32,
    pub default_fixed_font_size: u32,
    pub minimum_font_size: u32,
    pub minimum_logical_font_size: u32,
    pub default_encoding: CefString,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub webgl: State,
    pub background_color: u32,
    pub chrome_status_bubble: State,
    pub chrome_zoom_bubble: State,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            windowless_frame_rate: Default::default(),
            standard_font_family: Default::default(),
            fixed_font_family: Default::default(),
            serif_font_family: Default::default(),
            sans_serif_font_family: Default::default(),
            cursive_font_family: Default::default(),
            fantasy_font_family: Default::default(),
            default_font_size: Default::default(),
            default_fixed_font_size: Default::default(),
            minimum_font_size: Default::default(),
            minimum_logical_font_size: Default::default(),
            default_encoding: Default::default(),
            remote_fonts: State::STATE_DEFAULT,
            javascript: State::STATE_DEFAULT,
            javascript_close_windows: State::STATE_DEFAULT,
            javascript_access_clipboard: State::STATE_DEFAULT,
            javascript_dom_paste: State::STATE_DEFAULT,
            image_loading: State::STATE_DEFAULT,
            image_shrink_standalone_to_fit: State::STATE_DEFAULT,
            text_area_resize: State::STATE_DEFAULT,
            tab_to_links: State::STATE_DEFAULT,
            local_storage: State::STATE_DEFAULT,
            databases: State::STATE_DEFAULT,
            webgl: State::STATE_DEFAULT,
            background_color: Default::default(),
            chrome_status_bubble: State::STATE_DEFAULT,
            chrome_zoom_bubble: State::STATE_DEFAULT,
        }
    }
}

impl BrowserSettings {
    pub(crate) fn from_mut_ptr(raw: *mut cef_browser_settings_t) -> Self {
        Self::from(unsafe { &*raw })
    }

    pub fn into_raw(self) -> cef_browser_settings_t {
        cef_browser_settings_t {
            size: std::mem::size_of::<cef_browser_settings_t>(),
            windowless_frame_rate: self.windowless_frame_rate as c_int,
            standard_font_family: self.standard_font_family.get_raw(),
            fixed_font_family: self.fixed_font_family.get_raw(),
            serif_font_family: self.serif_font_family.get_raw(),
            sans_serif_font_family: self.sans_serif_font_family.get_raw(),
            cursive_font_family: self.cursive_font_family.get_raw(),
            fantasy_font_family: self.fantasy_font_family.get_raw(),
            default_font_size: self.default_font_size as c_int,
            default_fixed_font_size: self.default_fixed_font_size as c_int,
            minimum_font_size: self.minimum_font_size as c_int,
            minimum_logical_font_size: self.minimum_logical_font_size as c_int,
            default_encoding: self.default_encoding.get_raw(),
            remote_fonts: self.remote_fonts,
            javascript: self.javascript,
            javascript_close_windows: self.javascript_close_windows,
            javascript_access_clipboard: self.javascript_access_clipboard,
            javascript_dom_paste: self.javascript_dom_paste,
            image_loading: self.image_loading,
            image_shrink_standalone_to_fit: self.image_shrink_standalone_to_fit,
            text_area_resize: self.text_area_resize,
            tab_to_links: self.tab_to_links,
            local_storage: self.local_storage,
            databases: self.databases,
            webgl: self.webgl,
            background_color: self.background_color,
            chrome_status_bubble: self.chrome_status_bubble,
            chrome_zoom_bubble: self.chrome_zoom_bubble,
        }
    }
}

impl From<&cef_browser_settings_t> for BrowserSettings {
    fn from(raw: &cef_browser_settings_t) -> Self {
        BrowserSettings {
            windowless_frame_rate: raw.windowless_frame_rate as usize,
            standard_font_family: raw.standard_font_family.try_into().expect("Error converting standard_font_family string"),
            fixed_font_family: raw.fixed_font_family.try_into().expect("Error converting fixed_font_family string"),
            serif_font_family: raw.serif_font_family.try_into().expect("Error converting serif_font_family string"),
            sans_serif_font_family: raw.sans_serif_font_family.try_into().expect("Error converting sans_serif_font_family string"),
            cursive_font_family: raw.cursive_font_family.try_into().expect("Error converting cursive_font_family string"),
            fantasy_font_family: raw.fantasy_font_family.try_into().expect("Error converting fantasy_font_family string"),
            default_font_size: raw.default_font_size as u32,
            default_fixed_font_size: raw.default_fixed_font_size as u32,
            minimum_font_size: raw.minimum_font_size as u32,
            minimum_logical_font_size: raw.minimum_logical_font_size as u32,
            default_encoding: raw.default_encoding.try_into().expect("Error converting default_encoding string"),
            remote_fonts: raw.remote_fonts,
            javascript: raw.javascript,
            javascript_close_windows: raw.javascript_close_windows,
            javascript_access_clipboard: raw.javascript_access_clipboard,
            javascript_dom_paste: raw.javascript_dom_paste,
            image_loading: raw.image_loading,
            image_shrink_standalone_to_fit: raw.image_shrink_standalone_to_fit,
            text_area_resize: raw.text_area_resize,
            tab_to_links: raw.tab_to_links,
            local_storage: raw.local_storage,
            databases: raw.databases,
            webgl: raw.webgl,
            background_color: raw.background_color,
            chrome_status_bubble: raw.chrome_status_bubble,
            chrome_zoom_bubble: raw.chrome_zoom_bubble,
        }
    }
}

/// See [cef_browser_host_create_browser] for more documentation.
pub fn create_browser<T: Client>(
    window_info: WindowInfo,
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
) -> i32 {
    let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

    unsafe {
        cef_browser_host_create_browser(
            &window_info.into_raw(),
            client,
            &url.get_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
        )
    }
}

wrapper!(
    #[doc = "See [cef_browser_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct BrowserHost(cef_browser_host_t);
    pub fn send_external_begin_frame(&self);
    pub fn was_resized(&self);
);

impl BrowserHost {
    pub fn set_focus(&self, focus: bool) {
        if let Some(f) = self.0.set_focus {
            unsafe { f(self.0.get_raw(), focus.into()) };
        }
    }

    pub fn get_windowless_frame_rate(&self) -> i32 {
        self.0
            .get_windowless_frame_rate
            .map(|f| unsafe { f(self.0.get_raw()) })
            .unwrap_or(-1)
    }

    pub fn is_render_process_unresponsive(&self) -> bool {
        self.0
            .is_render_process_unresponsive
            .map(|f| unsafe { f(self.0.get_raw()) } > 0)
            .unwrap_or(true)
    }

    // pub fn send_mouse_move_event(&self) {
    //     let event = cef_mouse_event_t {
    //         x: 10,
    //         y: 10,
    //         modifiers: 0,
    //     };

    //     if let Some(f) = self.0.send_mouse_move_event {
    //         unsafe { f(self.0.get_raw(), &event, 0) }
    //     }
    // }

    pub fn invalidate(&self, type_: PaintElementType) {
        if let Some(f) = self.0.invalidate {
            unsafe { f(self.0.get_raw(), type_) };
        }
    }
}

wrapper!(
    #[doc = "See [cef_browser_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Browser(cef_browser_t);
    pub fn go_back(&self);
    pub fn go_forward(&self);
    pub fn reload(&self);
    pub fn reload_ignore_cache(&self);
    pub fn stop_load(&self);
);

impl Browser {
    pub fn get_host(&self) -> Option<BrowserHost> {
        self.0.get_host.and_then(|f| {
            let p = unsafe { f(self.0.get_raw()) };
            if p.is_null() {
                None
            } else {
                Some(unsafe { BrowserHost::from_raw(p) })
            }
        })
    }
}

/// See [cef_browser_host_create_browser] for more documentation.
pub fn create_browser_sync<T: Client>(
    window_info: WindowInfo,
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
) -> Browser {
    let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

    unsafe {
        let ptr = cef_browser_host_create_browser_sync(
            &window_info.into_raw(),
            client,
            &url.get_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
        );

        Browser::from_raw(ptr)
    }
}

wrapper!(
    #[doc = "See [cef_browser_view_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct BrowserView(cef_browser_view_t);
);

impl BrowserView {
    pub fn get_view(&self) -> View {
        unsafe { View(self.0.convert()) }
    }

    pub fn get_browser(&self) -> Option<Browser> {
        self.0.get_browser.and_then(|f| {
            let p = unsafe { f(self.0.get_raw()) };
            if p.is_null() {
                None
            } else {
                Some(unsafe { Browser::from_raw(p) })
            }
        })
    }
}

/// See [cef_browser_view_create] for more documentation.
pub fn create_browser_view<T: Client>(
    client: Option<T>,
    url: CefString,
    settings: BrowserSettings,
    // TODO delegate: *mut _cef_browser_view_delegate_t,
) -> BrowserView {
    let client = client.map(|c| c.into_raw()).unwrap_or(null_mut());

    let view = unsafe {
        cef_browser_view_create(
            client,
            &url.get_raw(),
            &settings.into_raw(),
            null_mut(),
            null_mut(),
            null_mut(),
        )
    };

    unsafe { BrowserView::from_raw(view) }
}