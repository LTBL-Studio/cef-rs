use cef_sys::cef_process_message_t;

use crate::{string::{CefString, CefStringError}, values::ListValue, wrapper};

pub type ProcessId = cef_sys::cef_process_id_t;

wrapper!(
    #[doc = "See [cef_process_message_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct ProcessMessage(cef_process_message_t);
);

impl ProcessMessage {
    pub fn is_valid(&self) -> bool {
        let is_valid = self.0.is_valid.unwrap();

        unsafe { is_valid(self.0.get_raw()) > 0 }
    }

    pub fn is_read_only(&self) -> bool {
        let is_read_only = self.0.is_read_only.unwrap();

        unsafe { is_read_only(self.0.get_raw()) > 0 }
    }

    pub fn copy(&self) -> Option<ProcessMessage> {
        self.0.copy.and_then(|copy| {
            let ptr = unsafe { copy(self.0.get_raw()) };
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { ProcessMessage::from_raw(ptr) })
            }
        })
    }
    
    pub fn get_name(&self) -> Result<CefString, CefStringError> {
        let get_name = self.0.get_name.unwrap();
        
        unsafe { CefString::from_userfree_cef(get_name(self.0.get_raw())) }
    }

    pub fn get_argument_list(&self) -> Option<ListValue> {
        self.0.get_argument_list.and_then(|get_argument_list| {
            let ptr = unsafe { get_argument_list(self.0.get_raw()) };
            if ptr.is_null() {
                None
            } else {
                Some(unsafe { ListValue::from_raw(ptr) })
            }
        })
    }
}

// TO DO: Implement cef_process_message_t methods
