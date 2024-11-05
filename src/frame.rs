use cef_sys::cef_frame_t;

use crate::{string::CefString, wrapper};

wrapper!(
    #[doc = "See [cef_frame_t] for more documentation."]
    #[derive(Debug)]
    pub struct Frame(cef_frame_t);
    pub fn undo(&self);
    pub fn redo(&self);
    pub fn cut(&self);
    pub fn copy(&self);
    pub fn paste(&self);
    pub fn del(&self);
    pub fn select_all(&self);
    pub fn view_source(&self);
);

impl Frame {
    pub fn load_url(&self, url: CefString) {
        if let Some(f) = self.0.load_url {
            unsafe { f(self.0.get_raw(), &url.get_raw()) };
        }
    }
}
