use cef_sys::{_cef_accelerated_paint_info_t, _cef_accelerated_paint_native_pixmap_plane_info_t, cef_point_t, cef_range_t, cef_rect_t, cef_screen_info_t};

pub type ColorType = cef_sys::cef_color_type_t;
pub type PaintElementType = cef_sys::cef_paint_element_type_t;
pub type CefAcceleratedPaintNativePixmapPlaneInfo = _cef_accelerated_paint_native_pixmap_plane_info_t;

/// See [cef_rect_t] for more documentation.
#[derive(Debug, Clone, Default)]
pub struct CefRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl CefRect {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_ptr(raw: *const cef_rect_t) -> Self {
        Self::from(unsafe { &*raw })
    }
    pub(crate) fn from(raw: &cef_rect_t) -> Self {
        CefRect {
            x: raw.x,
            y: raw.y,
            width: raw.width,
            height: raw.height,
        }
    }

    pub fn into_raw(self) -> cef_rect_t {
        cef_rect_t {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub(crate) fn from_array(count: usize, rects: *const cef_rect_t) -> Vec<CefRect> {
        let raw_rects = unsafe { core::slice::from_raw_parts(rects, count) };
        raw_rects.iter().map(Self::from).collect()
    }
}

/// See [cef_point_t] for more documentation.
#[derive(Debug, Clone, Default)]
pub struct CefPoint {
    pub x: i32,
    pub y: i32,
}

impl CefPoint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_raw(self) -> cef_point_t {
        cef_point_t {
            x: self.x,
            y: self.y,
        }
    }
}

/// See [cef_range_t] for more documentation.
#[derive(Debug, Clone, Default)]
pub struct CefRange {
    pub from: u32,
    pub to: u32,
}

impl CefRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_ptr(raw: *const cef_range_t) -> Self {
        Self::from(unsafe { &*raw })
    }

    pub(crate) fn from(raw: &cef_range_t) -> Self {
        CefRange {
            from: raw.from,
            to: raw.to,
        }
    }

    pub fn into_raw(self) -> cef_range_t {
        cef_range_t {
            from: self.from,
            to: self.to,
        }
    }
}

/// See [_cef_accelerated_paint_info_t] for more documentation.
#[derive(Debug, Clone)]
pub struct CefAcceleratedPaintInfo {
    pub planes: [CefAcceleratedPaintNativePixmapPlaneInfo; 4usize],
    pub plane_count: i32,
    pub modifier: u64,
    pub format: ColorType,
}

impl CefAcceleratedPaintInfo {
    pub(crate) fn from_ptr(raw: *const _cef_accelerated_paint_info_t) -> Self {
        Self::from(unsafe { &*raw })
    }
    pub(crate) fn from(raw: &_cef_accelerated_paint_info_t) -> Self {
        CefAcceleratedPaintInfo {
            planes: raw.planes,
            plane_count: raw.plane_count,
            modifier: raw.modifier,
            format: raw.format,
        }
    }

    pub fn into_raw(self) -> _cef_accelerated_paint_info_t {
        _cef_accelerated_paint_info_t {
            planes: self.planes,
            plane_count: self.plane_count,
            modifier: self.modifier,
            format: self.format,
        }
    }
}

/// See [cef_screen_info_t] for more documentation.
#[derive(Debug, Clone, Default)]
pub struct CefScreenInfo {
    pub device_scale_factor: f32,
    pub depth: i32,
    pub depth_per_component: i32,
    pub is_monochrome: bool,
    pub rect: CefRect,
    pub available_rect: CefRect,
}

impl CefScreenInfo {
    pub(crate) fn from(raw: &cef_screen_info_t) -> Self {
        CefScreenInfo {
            device_scale_factor: raw.device_scale_factor,
            depth: raw.depth,
            depth_per_component: raw.depth_per_component,
            is_monochrome: raw.is_monochrome > 0,
            rect: CefRect::from(&raw.rect),
            available_rect: CefRect::from(&raw.available_rect),
        }
    }

    pub(crate) fn from_mut_ptr(raw: *mut cef_screen_info_t) -> Self {
        Self::from(unsafe { &*raw })
    }

    pub fn into_raw(self) -> cef_screen_info_t {
        cef_screen_info_t {
            device_scale_factor: self.device_scale_factor,
            depth: self.depth,
            depth_per_component: self.depth_per_component,
            is_monochrome: self.is_monochrome.into(),
            rect: self.rect.into_raw(),
            available_rect: self.available_rect.into_raw(),
        }
    }
}
