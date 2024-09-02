use cef_sys::{cef_dictionary_value_create, cef_dictionary_value_t, cef_list_value_create, cef_list_value_t};

use crate::wrapper;

wrapper!(
    #[doc = "See [cef_list_value_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct ListValue(cef_list_value_t);
);

impl Default for ListValue {
    fn default() -> Self {
        unsafe { ListValue::from_raw(cef_list_value_create()) }
    }
    
    // TO DO: Implement cef_list_value_t methods
}

wrapper!(
    #[doc = "See [cef_dictionary_value_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct DictionaryValue(cef_dictionary_value_t);
);

impl Default for DictionaryValue {
    fn default() -> Self {
        unsafe { DictionaryValue::from_raw(cef_dictionary_value_create()) }
    }

    // TO DO: Implement cef_dictionary_value_t methods
}
