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
    cef_string_list_alloc, cef_string_list_append, cef_string_list_clear, cef_string_list_copy, cef_string_list_free, cef_string_list_size, cef_string_list_t, cef_string_list_value, cef_string_map_alloc, cef_string_map_append, cef_string_map_clear, cef_string_map_find, cef_string_map_free, cef_string_map_key, cef_string_map_size, cef_string_map_t, cef_string_map_value, cef_string_t, cef_string_userfree_utf16_t, cef_string_utf16_t
};
use std::collections::HashMap;
use std::fmt::Display;
use std::ptr::null_mut;
use widestring::error::ContainsNul;
use widestring::U16CString;

#[derive(Debug)]
pub enum CefStringError {
    NullPointer,
    Conversion,
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
                    .map(CefString)
                    .map_err(|_| CefStringError::Conversion)
            }
        }
    }

    /// Create a `CefString` from raw `cef_string_userfree_utf16_t` pointer. If the pointer is null or it fails
    /// to convert to `U16CString`, this method will returns a [CefStringError].
    pub unsafe fn from_userfree_cef(
        ptr: cef_string_userfree_utf16_t,
    ) -> Result<CefString, CefStringError> {
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

impl From<&str> for CefString {
    // type Err = CefStringError;

    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     U16CString::from_str(s).map(CefString).map_err(|_| CefStringError::Conversion)
    // }

    fn from(value: &str) -> Self {
        Self(U16CString::from_str(value).expect("Failed to create CefString from str."))
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
            if let Ok(v) = CefString::from_raw(value) {
                res.push(v.to_string())
            }
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

/// Helper type to deal with Cef string list.
#[derive(Debug)]
pub struct CefStringList(cef_string_list_t);

impl Default for CefStringList {
    fn default() -> Self {
        Self(unsafe { cef_string_list_alloc() })
    }
}

impl CefStringList {
    /// Returns the number of elements in the list
    pub fn size(&self) -> usize {
        unsafe { cef_string_list_size(self.0) }
    }

    /// Returns the value at the supplied index
    pub fn get(&self, index: usize) -> Option<CefString> {
        let value = CefString::default();
        if unsafe { cef_string_list_value(self.0, index, &mut value.get_raw()) } > 0 {
            Some(value)
        } else {
            None
        }
    }

    /// Appends a new value at the end of the list
    pub fn append(&mut self, value: CefString) {
        unsafe { cef_string_list_append(self.0, &value.get_raw()) }
    }

    /// Clears the list
    pub fn clear(&mut self) {
        unsafe { cef_string_list_clear(self.0) }
    }

    pub(crate) fn get_raw_mut(&mut self) -> cef_string_list_t {
        self.0
    }
}

impl Clone for CefStringList {
    fn clone(&self) -> Self {
        Self(unsafe { cef_string_list_copy(self.0) })
    }
}

impl Drop for CefStringList {
    fn drop(&mut self) {
        unsafe { cef_string_list_free(self.0) };
    }
}

/// Helper type to deal with Cef oredered string map.
#[derive(Debug)]
pub struct CefStringMap(cef_string_map_t);

impl Default for CefStringMap {
    fn default() -> Self {
        Self(unsafe { cef_string_map_alloc() })
    }
}

impl CefStringMap {
    /// Returns the number of elements in the map
    pub fn size(&self) -> usize {
        unsafe { cef_string_map_size(self.0) }
    }

    /// Returns the value assigned to the supplied key
    pub fn get(&self, key: CefString) -> Option<CefString> {
        let value = CefString::default();
        if unsafe { cef_string_map_find(self.0, &key.get_raw(), &mut value.get_raw()) } > 0 {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the value at the supplied index
    pub fn get_index(&self, index: usize) -> Option<CefString> {
        let value = CefString::default();
        if unsafe { cef_string_map_value(self.0, index, &mut value.get_raw()) } > 0 {
            Some(value)
        } else {
            None
        }
    }

    /// Returns the key at the supplied index
    pub fn get_key_at_index(&self, index: usize) -> Option<CefString> {
        let key = CefString::default();
        if unsafe { cef_string_map_key(self.0, index, &mut key.get_raw()) } > 0 {
            Some(key)
        } else {
            None
        }
    }

    /// Appends a new key/value pair at the end of the string map. If the key exists, overwrite the existing value with a new value without changing the pair order and returns the old value.
    pub fn append(&mut self, key: CefString, value: CefString) -> Option<CefString> {
        let raw_key = &key.get_raw();
        let old_value = self.get(key);
        unsafe { cef_string_map_append(self.0, raw_key, &value.get_raw()) };
        old_value
    }

    /// Clears the map
    pub fn clear(&mut self) {
        unsafe { cef_string_map_clear(self.0) }
    }

    pub(crate) fn get_raw_mut(&mut self) -> cef_string_map_t {
        self.0
    }
}

impl Drop for CefStringMap {
    fn drop(&mut self) {
        unsafe { cef_string_map_free(self.0) };
    }
}
