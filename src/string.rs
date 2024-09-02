//! String module
//!
//! cef-rs defines its own type [`CefString`] which is a new
//! type of [`widestring::U16CString`] to handle everything around strings.
//! Including converting from/to raw [`cef_string_utf16_t`],
//! converting from/to Rust string types. Every cef-rs types should also use [`CefString`] as
//! interface in most of the cases. Raw cef string [`cef_string_utf16_t`] is a UTF-16 C String,
//! but it also has a version [`cef_string_userfree_utf16_t`] that users are responsible
//! for free it manually.

use cef_sys::{
    cef_string_list_t, cef_string_map_t, cef_string_userfree_utf16_t, cef_string_utf16_t,
};
use widestring::error::ContainsNul;
use std::collections::HashMap;
use std::fmt::Display;
use std::ptr::null_mut;
use widestring::U16CString;

#[derive(Debug)]
pub enum CefStringError {
    NullPointer,
    Conversion
}

/// Helper type to deal with Cef string. It's essentially an UTF-16 C string.
#[derive(Debug, Default, Clone)]
pub struct CefString(pub U16CString);

impl CefString {
    pub fn new(s: &str) -> Self {
        Self(U16CString::from_str(s).expect("Failed to create CefString from str."))
    }

    /// Create a `CefString` from raw `cef_string_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns a [CefStringError].
    pub unsafe fn from_raw(ptr: *const cef_string_utf16_t) -> Result<CefString, CefStringError> {
        if ptr.is_null() {
            Err(CefStringError::NullPointer)
        } else {
            // It's a smart pointer, so cef retains ownership and will call the dtor
            unsafe {
                U16CString::from_ptr((*ptr).str_, (*ptr).length)
                    .map(CefString).map_err(|_| CefStringError::Conversion)
            }
        }
    }

    /// Create a `CefString` from raw `cef_string_userfree_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns a [CefStringError].
    pub unsafe fn from_userfree_cef(ptr: cef_string_userfree_utf16_t) -> Result<CefString, CefStringError> {
        let res = unsafe { Self::from_raw(ptr) }?;
        unsafe {
            cef_sys::cef_string_userfree_utf16_free(ptr);
        }
        Ok(res)
    }

    /// Get raw [cef_string_utf16_t] which doesn't have the ownership of the value.
    /// This should be used when you need to pass the `*const cef_string_utf16_t` to the function.
    pub fn get_raw(&self) -> cef_string_utf16_t {
        cef_string_utf16_t {
            length: self.0.len(),
            str_: self.0.as_ptr() as *mut _,
            dtor: None,
        }
    }
}

impl TryFrom<cef_string_utf16_t> for CefString {
    type Error = ContainsNul<u16>;

    fn try_from(value: cef_string_utf16_t) -> Result<Self, Self::Error> {
        unsafe { U16CString::from_ptr(value.str_, value.length).map(CefString) }
    }
}

impl Display for CefString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

pub unsafe fn parse_string_list(ptr: cef_string_list_t) -> Vec<String> {
    let count = cef_sys::cef_string_list_size(ptr);
    let mut res = Vec::with_capacity(count);
    for i in 0..count {
        let value = null_mut();
        if cef_sys::cef_string_list_value(ptr, i, value) > 0 {
            if let Ok(v) = CefString::from_raw(value) { res.push(v.to_string()) }
        }
    }
    res
}

pub unsafe fn parse_string_map(ptr: cef_string_map_t) -> HashMap<String, String> {
    let count = cef_sys::cef_string_map_size(ptr);
    let mut res = HashMap::with_capacity(count);
    for i in 0..count {
        let key = null_mut();
        let value = null_mut();
        cef_sys::cef_string_map_key(ptr, i, key);
        cef_sys::cef_string_map_value(ptr, i, value);

        let _ = CefString::from_raw(key)
            .map(|k| CefString::from_raw(value).map(|v| res.insert(k.to_string(), v.to_string())));
    }
    res
}
