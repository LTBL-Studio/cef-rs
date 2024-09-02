use cef_sys::cef_frame_t;

use crate::wrapper;

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