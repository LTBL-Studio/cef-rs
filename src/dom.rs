use cef_sys::cef_domnode_t;

use crate::wrapper;

wrapper!(
    #[doc = "See [cef_domnode_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct DomNode(cef_domnode_t);
);

// TO DO: Implement cef_domnode_t methods
