use cef_sys::cef_v8context_t;

use crate::wrapper;

wrapper!(
    #[doc = "See [cef_v8context_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct V8Context(cef_v8context_t);
);

// TO DO: Implement cef_v8context_t methods
