use cef_sys::cef_v8stack_trace_t;

use crate::wrapper;

wrapper!(
    #[doc = "See [cef_v8stack_trace_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct V8StackTrace(cef_v8stack_trace_t);
);

// TO DO: Implement cef_v8stack_trace_t methods
