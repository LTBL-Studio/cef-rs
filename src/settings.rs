use std::ffi::c_int;

use cef_sys::{cef_color_t, cef_settings_t};

use crate::{string::CefString, LogItems, LogSeverity};

/// See [cef_settings_t] for more documentation.
#[derive(Debug, Default, Clone)]
pub struct Settings {
    pub no_sandbox: bool,
    pub browser_subprocess_path: CefString,
    pub framework_dir_path: CefString,
    pub main_bundle_path: CefString,
    pub chrome_runtime: bool,
    pub multi_threaded_message_loop: bool,
    pub external_message_pump: bool,
    pub windowless_rendering_enabled: bool,
    pub command_line_args_disabled: bool,
    pub cache_path: CefString,
    pub root_cache_path: CefString,
    pub persist_session_cookies: bool,
    pub persist_user_preferences: bool,
    pub user_agent: CefString,
    pub user_agent_product: CefString,
    pub locale: CefString,
    pub log_file: CefString,
    pub log_severity: LogSeverity,
    pub log_items: LogItems,
    pub javascript_flags: CefString,
    pub resources_dir_path: CefString,
    pub locales_dir_path: CefString,
    pub pack_loading_disabled: bool,
    pub remote_debugging_port: u32,
    pub uncaught_exception_stack_size: u32,
    pub background_color: u32,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: bool,
    pub chrome_policy_id: CefString,
    pub chrome_app_icon_id: i32,
}

impl Settings {
    pub(crate) fn from_mut_ptr(raw: *mut cef_settings_t) -> Self {
        Self::from(unsafe { &*raw })
    }

    pub fn get_raw(&self) -> cef_settings_t {
        cef_settings_t {
            size: std::mem::size_of::<cef_settings_t>(),
            no_sandbox: self.no_sandbox as c_int,
            browser_subprocess_path: self.browser_subprocess_path.get_raw(),
            framework_dir_path: self.framework_dir_path.get_raw(),
            main_bundle_path: self.main_bundle_path.get_raw(),
            chrome_runtime: self.chrome_runtime as c_int,
            multi_threaded_message_loop: self.multi_threaded_message_loop as c_int,
            external_message_pump: self.external_message_pump as c_int,
            windowless_rendering_enabled: self.windowless_rendering_enabled as c_int,
            command_line_args_disabled: self.command_line_args_disabled as c_int,
            cache_path: self.cache_path.get_raw(),
            root_cache_path: self.root_cache_path.get_raw(),
            persist_session_cookies: self.persist_session_cookies as c_int,
            persist_user_preferences: self.persist_user_preferences as c_int,
            user_agent: self.user_agent.get_raw(),
            user_agent_product: self.user_agent_product.get_raw(),
            locale: self.locale.get_raw(),
            log_file: self.log_file.get_raw(),
            log_severity: self.log_severity,
            log_items: self.log_items,
            javascript_flags: self.javascript_flags.get_raw(),
            resources_dir_path: self.resources_dir_path.get_raw(),
            locales_dir_path: self.locales_dir_path.get_raw(),
            pack_loading_disabled: self.pack_loading_disabled as c_int,
            remote_debugging_port: self.remote_debugging_port as c_int,
            uncaught_exception_stack_size: self.uncaught_exception_stack_size as c_int,
            background_color: self.background_color as cef_color_t,
            accept_language_list: self.accept_language_list.get_raw(),
            cookieable_schemes_list: self.cookieable_schemes_list.get_raw(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults as c_int,
            chrome_policy_id: self.chrome_policy_id.get_raw(),
            chrome_app_icon_id: self.chrome_app_icon_id,
        }
    }
}

impl From<&cef_settings_t> for Settings {
    fn from(raw: &cef_settings_t) -> Self {
        Settings {
            no_sandbox: raw.no_sandbox > 0,
            browser_subprocess_path: raw.browser_subprocess_path.try_into().expect("Error converting browser_subprocess_path string"),
            framework_dir_path: raw.framework_dir_path.try_into().expect("Error converting framework_dir_path string"),
            main_bundle_path: raw.main_bundle_path.try_into().expect("Error converting main_bundle_path string"),
            chrome_runtime: raw.chrome_runtime > 0,
            multi_threaded_message_loop: raw.multi_threaded_message_loop > 0,
            external_message_pump: raw.external_message_pump > 0,
            windowless_rendering_enabled: raw.windowless_rendering_enabled > 0,
            command_line_args_disabled: raw.command_line_args_disabled > 0,
            cache_path: raw.cache_path.try_into().expect("Error converting cache_path string"),
            root_cache_path: raw.root_cache_path.try_into().expect("Error converting root_cache_path string"),
            persist_session_cookies: raw.persist_session_cookies > 0,
            persist_user_preferences: raw.persist_user_preferences > 0,
            user_agent: raw.user_agent.try_into().expect("Error converting user_agent string"),
            user_agent_product: raw.user_agent_product.try_into().expect("Error converting user_agent_product string"),
            locale: raw.locale.try_into().expect("Error converting locale string"),
            log_file: raw.log_file.try_into().expect("Error converting log_file string"),
            log_severity: raw.log_severity,
            log_items: raw.log_items,
            javascript_flags: raw.javascript_flags.try_into().expect("Error converting javascript_flags string"),
            resources_dir_path: raw.resources_dir_path.try_into().expect("Error converting resources_dir_path string"),
            locales_dir_path: raw.locales_dir_path.try_into().expect("Error converting locales_dir_path string"),
            pack_loading_disabled: raw.pack_loading_disabled > 0,
            remote_debugging_port: raw.remote_debugging_port as u32,
            uncaught_exception_stack_size: raw.uncaught_exception_stack_size as u32,
            background_color: raw.background_color as cef_color_t,
            accept_language_list: raw.accept_language_list.try_into().expect("Error converting accept_language_list string"),
            cookieable_schemes_list: raw.cookieable_schemes_list.try_into().expect("Error converting cookieable_schemes_list string"),
            cookieable_schemes_exclude_defaults: raw.cookieable_schemes_exclude_defaults > 0,
            chrome_policy_id: raw.chrome_policy_id.try_into().expect("Error converting chrome_policy_id string"),
            chrome_app_icon_id: raw.chrome_app_icon_id,
        }
    }
}