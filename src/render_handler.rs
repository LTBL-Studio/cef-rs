use std::ptr::null_mut;

use cef_sys::{
    cef_accelerated_paint_info_t, cef_accessibility_handler_t, cef_browser_t, cef_drag_data_t,
    cef_drag_operations_mask_t, cef_paint_element_type_t, cef_range_t, cef_rect_t,
    cef_render_handler_t, cef_screen_info_t, cef_string_t, cef_text_input_mode_t,
};

use crate::{
    rc::RcImpl, render_utils::{CefAcceleratedPaintInfo, CefPoint, CefRange, CefRect, CefScreenInfo, PaintElementType}, string::CefString, Browser, DragOperationsMask, TextInputMode
};

/// See [cef_render_handler_t] for more documentation.
pub trait RenderHandler: Sized {
    //get_accessibility_handler
    fn get_root_screen_rect(&self, _browser: &Browser) -> Option<CefRect> {
        None
    }
    fn get_view_rect(&self, browser: &Browser) -> CefRect;
    fn get_screen_point(&self, _browser: &Browser, _view: CefPoint) -> Option<CefPoint> {
        println!("get_screen_point");
        None
    }
    fn get_screen_info(&self, _browser: &Browser, _screen_info: CefScreenInfo) -> bool {
        false
    }
    //get_screen_info:
    fn on_popup_show(&self, _browser: &Browser, _show: bool) {
        panic!()
    }
    fn on_popup_size(&self, _browser: &Browser, _rect: CefRect) {
        panic!()
    }
    fn on_paint(
        &self,
        _browser: &Browser,
        _type_: PaintElementType,
        _dirty_rects: &[CefRect],
        _bytes: &[u8],
        _width: i32,
        _height: i32,
    ) {
        println!("on_paint");
    }
    fn on_accelerated_paint(
        &self,
        _browser: &Browser,
        _type_: PaintElementType,
        _dirty_rects: &[CefRect],
        _info: CefAcceleratedPaintInfo,
    ) {
        println!("on_accelerated_paint");
    }
    fn update_drag_cursor(&self, _browser: &Browser, _operation: DragOperationsMask) {
        panic!()
    }
    fn on_scroll_offset_changed(&self, _browser: &Browser, _x: f64, _y: f64) {
        panic!()
    }
    fn on_ime_composition_range_changed(
        &self,
        _browser: &Browser,
        _selected_range: CefRange,
        _character_bounds: &[CefRect],
    ) {
        panic!()
    }
    fn on_text_selection_changed(
        &self,
        _browser: &Browser,
        _selected_text: String,
        _selected_range: CefRange,
    ) {
        panic!()
    }
    fn on_virtual_keyboard_requested(&self, _browser: &Browser, _input_mode: TextInputMode) {
        println!("{:?}", _input_mode);
    }

    fn get_raw(&self) -> *mut cef_render_handler_t {
        let mut object: cef_render_handler_t = unsafe { std::mem::zeroed() };

        object.get_accessibility_handler = Some(get_accessibility_handler::<Self>);
        object.get_root_screen_rect = Some(get_root_screen_rect::<Self>);
        object.get_view_rect = Some(get_view_rect::<Self>);
        object.get_screen_point = Some(get_screen_point::<Self>);
        object.get_screen_info = Some(get_screen_info::<Self>);
        object.on_popup_show = Some(on_popup_show::<Self>);
        object.on_popup_size = Some(on_popup_size::<Self>);
        object.on_paint = Some(on_paint::<Self>);
        object.on_accelerated_paint = Some(on_accelerated_paint::<Self>);
        object.start_dragging = Some(start_dragging::<Self>);
        object.update_drag_cursor = Some(update_drag_cursor::<Self>);
        object.on_scroll_offset_changed = Some(on_scroll_offset_changed::<Self>);
        object.on_ime_composition_range_changed = Some(on_ime_composition_range_changed::<Self>);
        object.on_text_selection_changed = Some(on_text_selection_changed::<Self>);
        object.on_virtual_keyboard_requested = Some(on_virtual_keyboard_requested::<Self>);

        RcImpl::new(object, self) as *mut _
    }
}

impl RenderHandler for () {
    fn get_root_screen_rect(&self, _browser: &Browser) -> Option<CefRect> {
        None
    }

    fn get_view_rect(&self, _browser: &Browser) -> CefRect {
        CefRect::default()
    }
}

extern "C" fn get_accessibility_handler<R: RenderHandler>(
    _this: *mut cef_render_handler_t,
) -> *mut cef_accessibility_handler_t {
    // TODO
    null_mut()
}

extern "C" fn get_root_screen_rect<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    rect: *mut cef_rect_t,
) -> ::std::os::raw::c_int {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    println!("Is root screen rect null ? {}", rect.is_null());

    if let Some(res) = client.interface.get_root_screen_rect(&browser) {
        if !rect.is_null() {
            unsafe {
                (*rect).x = res.x;
                (*rect).y = res.y;
                (*rect).width = res.width as i32;
                (*rect).height = res.height as i32;
            }
        unsafe{println!("{:?}", *rect);}

        }
        1
    } else {
        0
    }
}
extern "C" fn get_view_rect<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    rect: *mut cef_rect_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let res = client.interface.get_view_rect(&browser);
    println!("Is view rect null ? {}", rect.is_null());
    if !rect.is_null() {
        unsafe {
            (*rect).x = res.x;
            (*rect).y = res.y;
            (*rect).width = res.width as i32;
            (*rect).height = res.height as i32;
        }

        unsafe{println!("{:?}", *rect);}
    }
}
extern "C" fn get_screen_point<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    view_x: ::std::os::raw::c_int,
    view_y: ::std::os::raw::c_int,
    screen_x: *mut ::std::os::raw::c_int,
    screen_y: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let view_point = CefPoint {
        x: view_x,
        y: view_y,
    };

    let screen_point = client.interface.get_screen_point(&browser, view_point);
    if let Some(screen_point) = screen_point {
        unsafe {
            *screen_x = screen_point.x;
            *screen_y = screen_point.y;
        }
        1
    } else {
        0
    }
}

extern "C" fn get_screen_info<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    screen_info: *mut cef_screen_info_t,
) -> ::std::os::raw::c_int {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client.interface.get_screen_info(&browser, CefScreenInfo::from_mut_ptr(screen_info)).into()
}

extern "C" fn on_popup_show<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    show: ::std::os::raw::c_int,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client.interface.on_popup_show(&browser, show > 0);
}

extern "C" fn on_popup_size<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    rect: *const cef_rect_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let rect = CefRect::from_ptr(rect);
    client.interface.on_popup_size(&browser, rect);
}

extern "C" fn on_paint<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    type_: cef_paint_element_type_t,
    dirty_rects_count: usize,
    dirty_rects: *const cef_rect_t,
    buffer: *const ::std::os::raw::c_void,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) {
    println!("On paint called");
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let dirty_rects = CefRect::from_array(dirty_rects_count, dirty_rects);
    let bytes =
        unsafe { core::slice::from_raw_parts(buffer as *const u8, (width * height * 4) as usize) };

    client
        .interface
        .on_paint(&browser, type_, &dirty_rects, bytes, width, height);
}

extern "C" fn on_accelerated_paint<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    type_: cef_paint_element_type_t,
    dirty_rects_count: usize,
    dirty_rects: *const cef_rect_t,
    info: *const cef_accelerated_paint_info_t,
) {
    println!("On accelerated paint called");

    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let dirty_rects = CefRect::from_array(dirty_rects_count, dirty_rects);
    let info = CefAcceleratedPaintInfo::from_ptr(info);

    client
        .interface
        .on_accelerated_paint(&browser, type_, &dirty_rects, info);
}

extern "C" fn start_dragging<R: RenderHandler>(
    _this: *mut cef_render_handler_t,
    _browser: *mut cef_browser_t,
    _drag_data: *mut cef_drag_data_t,
    _allowed_ops: cef_drag_operations_mask_t,
    _x: ::std::os::raw::c_int,
    _y: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    // TODO
    0
}

extern "C" fn update_drag_cursor<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    operation: cef_drag_operations_mask_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let operation = DragOperationsMask::from(operation);

    client.interface.update_drag_cursor(&browser, operation);
}

extern "C" fn on_scroll_offset_changed<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    x: f64,
    y: f64,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };

    client.interface.on_scroll_offset_changed(&browser, x, y);
}

extern "C" fn on_ime_composition_range_changed<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    selected_range: *const cef_range_t,
    character_bounds_count: usize,
    character_bounds: *const cef_rect_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let selected_range = CefRange::from_ptr(selected_range);
    let character_bounds = CefRect::from_array(character_bounds_count, character_bounds);

    client
        .interface
        .on_ime_composition_range_changed(&browser, selected_range, &character_bounds);
}

extern "C" fn on_text_selection_changed<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    selected_text: *const cef_string_t,
    selected_range: *const cef_range_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    let selected_text =
        unsafe { CefString::from_raw(selected_text).expect("Error executing CefString::from_raw") };
    let selected_range = CefRange::from_ptr(selected_range);

    client
        .interface
        .on_text_selection_changed(&browser, selected_text.to_string(), selected_range);
}

extern "C" fn on_virtual_keyboard_requested<R: RenderHandler>(
    this: *mut cef_render_handler_t,
    browser: *mut cef_browser_t,
    input_mode: cef_text_input_mode_t,
) {
    let client: &mut RcImpl<_, R> = RcImpl::get(this);
    let browser = unsafe { Browser::from_raw(browser) };
    client
        .interface
        .on_virtual_keyboard_requested(&browser, input_mode);
}
