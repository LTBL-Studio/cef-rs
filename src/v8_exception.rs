use cef_sys::cef_v8exception_t;

use crate::{
    string::{CefString, CefStringError},
    wrapper,
};

wrapper!(
    #[doc = "See [cef_v8exception_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct V8Exception(cef_v8exception_t);
);

impl V8Exception {
    pub fn get_message(&self) -> Result<CefString, CefStringError> {
        let get_message = self.0.get_message.unwrap();
        unsafe { CefString::from_userfree_cef(get_message(self.0.get_raw())) }
    }
}
