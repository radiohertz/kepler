use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub headers: HashMap<String, String>,
}

impl Request {
    /// Create a new Request context.
    ///
    /// ```
    /// let r = Request::new();
    pub fn new(headers: HashMap<String, String>) -> Self {
        Request { headers }
    }
}
