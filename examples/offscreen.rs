use std::{path::Path, sync::Arc, thread::sleep, time::Duration};

use cef::{
    args::Args, client::Client, life_span_handler::LifeSpanHandler, load_handler::LoadHandler, render_handler::RenderHandler, render_utils::{CefRect, PaintElementType}, string::CefString, thread::{currently_on, post_task, Task, ThreadId}, App, Browser, BrowserSettings, LogSeverity, Settings, WindowInfo
};

#[derive(Debug, Clone, Copy)]
struct Application;

impl App for Application {}

#[derive(Debug)]
struct DemoLifeSpanHandler;

impl LifeSpanHandler for DemoLifeSpanHandler {
    fn on_after_created(&self, _browser: &Browser) {
        println!("on_after_created");
    }

    fn do_close(&self, _browser: &Browser) -> bool {
        println!("do_close");
        false
    }

    fn on_before_close(&self, _browser: &Browser) {
        println!("on_before_close");
    }
}

#[derive(Debug)]
struct DemoLoadHandler;

impl LoadHandler for DemoLoadHandler {
    fn on_loading_state_change(
        &self,
        _browser: &Browser,
        _is_loading: bool,
        _can_go_back: bool,
        _can_go_forward: bool,
    ) {
        println!("on_loading_state_change: _browser {:?}, is_loading: {}, _can_go_back: {}, _can_go_forward: {}", _browser, _is_loading, _can_go_back, _can_go_forward);
    }

    fn on_load_start(
        &self,
        _browser: &Browser,
        _frame: *mut cef_sys::cef_frame_t,
        _transition_type: cef::TransitionType,
    ) {
        println!(
            "on_load_start: _browser {:?}, _transition_type: {:?}",
            _browser, _transition_type
        );
    }

    fn on_load_end(
        &self,
        browser: &Browser,
        _frame: *mut cef_sys::cef_frame_t,
        _http_status_code: i32,
    ) {
        println!(
            "on_load_end: _browser {:?}, _http_status_code: {:?}",
            browser, _http_status_code
        );

        // if let Some(mut host) = browser.get_host() {
        //     host.set_focus(true);
        //     host.was_resized();
        // }
    }

    fn on_load_error(
        &self,
        _browser: &Browser,
        _frame: *mut cef_sys::cef_frame_t,
        _error_code: cef::ErrorCode,
        _error_text: CefString,
        _failed_url: CefString,
    ) {
        println!(
            "on_load_error: _browser {:?}, _error_code: {:?}, _error_text: {}, _failed_url: {}",
            _browser, _error_code, _error_text, _failed_url
        );
    }
}

#[derive(Debug)]
struct DemoRenderHandler;

impl RenderHandler for DemoRenderHandler {
    fn get_root_screen_rect(&self, _browser: &Browser) -> Option<CefRect> {
        println!("Get root screen rect");
        Some(CefRect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        })
    }

    fn get_view_rect(&self, _browser: &Browser) -> CefRect {
        println!("Get view rect");

        CefRect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        }
    }

    fn get_screen_point(
        &self,
        _browser: &Browser,
        _view: cef::render_utils::CefPoint,
    ) -> Option<cef::render_utils::CefPoint> {
        println!("Get screen point");

        None
    }

    fn get_screen_info(
        &self,
        _browser: &Browser,
        _screen_info: cef::render_utils::CefScreenInfo,
    ) -> bool {
        println!("{_screen_info:?}");

        true
    }

    fn on_paint(
        &self,
        _browser: &cef::Browser,
        _type_: PaintElementType,
        _dirty_rects: &[cef::render_utils::CefRect],
        _bytes: &[u8],
        _width: i32,
        _height: i32,
    ) {
        println!("Paint event !");

        // println!("{_bytes:?}");
        // panic!();
    }

    fn on_accelerated_paint(
        &self,
        _browser: &cef::Browser,
        _type_: PaintElementType,
        _dirty_rects: &[cef::render_utils::CefRect],
        _info: cef::render_utils::CefAcceleratedPaintInfo,
    ) {
        println!("Accelerated paint event !");
        // panic!();
    }
}

#[derive(Debug)]
struct DemoClient {
    render_handler: Arc<DemoRenderHandler>,
    load_handler: Arc<DemoLoadHandler>,
    life_span_handler: Arc<DemoLifeSpanHandler>,
}

impl Client for DemoClient {
    type RenderHandler = DemoRenderHandler;
    type LoadHandler = DemoLoadHandler;
    type LifeSpanHandler = DemoLifeSpanHandler;

    fn get_render_handler(&self) -> Option<Arc<DemoRenderHandler>> {
        Some(self.render_handler.clone())
    }

    fn get_load_handler(&self) -> Option<Arc<DemoLoadHandler>> {
        Some(self.load_handler.clone())
    }

    fn get_life_span_handler(&self) -> Option<Arc<DemoLifeSpanHandler>> {
        Some(self.life_span_handler.clone())
    }
}

struct BrowserTask;

impl Task for BrowserTask {
    fn execute(&self) {
        let browser_settings = BrowserSettings {
            windowless_frame_rate: 60,
            // webgl: State::STATE_ENABLED,
            ..Default::default()
        };
        
        let client = DemoClient {
            render_handler: Arc::new(DemoRenderHandler),
            load_handler: Arc::new(DemoLoadHandler),
            life_span_handler: Arc::new(DemoLifeSpanHandler),
        };

        println!("Client created");
        // let url = CefString::new("https://www.google.com");
        let url = CefString::new("https://developer.mozilla.org/fr/docs/Web/CSS/animation");

        // let browser_view = dbg!(cef::create_browser_view(
        //     Some(client),
        //     url,
        //     browser_settings
        // ));

        // let delegate = DemoWindow { browser_view };

        // let x = dbg!(cef::create_top_level_window(delegate));
        let window_info = WindowInfo {
            windowless_rendering_enabled: true,
            external_begin_frame_enabled: true,
            bounds: CefRect {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
            shared_texture_enabled: true,
            ..Default::default()
        };
        // window_info.shared_texture_enabled = true;
        // window_info.external_begin_frame_enabled = true;
        println!("Try create browser");

        let browser = dbg!(cef::create_browser_sync(
            window_info,
            Some(client),
            url,
            browser_settings
        ));

        if let Some(host) = browser.get_host() {
            loop {
                // let task = MessageTask;

                // println!("Loop");
                cef::do_message_loop_work();
                // host.invalidate(PaintElementType::PET_VIEW);
                // host.send_mouse_move_event();
                // println!("framerate: {}", host.get_windowless_frame_rate());
                // println!("{}", host.is_render_process_unresponsive());
                host.send_external_begin_frame();
                sleep(Duration::from_millis(33));
            }
        }

        println!("Browser created");
    }
}

fn main() {
    let args = Args::new(std::env::args());
    // dbg!(&args);
    let app = Application;
    let settings = Settings {
        // log_severity: LogSeverity::LOGSEVERITY_VERBOSE,
        log_severity: LogSeverity::LOGSEVERITY_DEFAULT,
        windowless_rendering_enabled: true,
        // external_message_pump: true,
        command_line_args_disabled: false,
        // multi_threaded_message_loop: true,
        // no_sandbox: true,
        locale: CefString::new("en-GB"),
        // resources_dir_path: CefString::new(&Path::new("/Absolute/Path/To/Resources").as_os_str().to_string_lossy()),
        // locales_dir_path: CefString::new(&Path::new("/Absolute/Path/To/locales").as_os_str().to_string_lossy()),
        // root_cache_path: CefString::new(&Path::new("/Absolute/Path/To/cache").as_os_str().to_string_lossy()),
        ..Default::default()
    };
    dbg!(cef::execute_process::<Application>(&args, Some(app)));
    dbg!(cef::initialize::<Application>(&args, &settings, Some(app)));

    let task = BrowserTask;
    if currently_on(ThreadId::TID_UI) {
        println!("Execute task immediately");
        task.execute();
    } else {
        println!("Post task");
        dbg!(post_task(ThreadId::TID_UI, task));
    }

    // let window_info = WindowInfo::new();

    // cef::do_message_loop_work();
    cef::run_message_loop();
    // dbg!(x.has_one_ref());

    println!("Coucou");

    cef::shutdown();
}
