use std::fmt;
use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub uri: String,
    pub http_version: String,
    // TODO: what's better: HashMap or Array of Structures(key, value)
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method:\n{}\nuri:\n{}\nhttp_version:\n{}\nheaders:\n{:?}\nbody:\n{}\n",
               self.method, self.uri, self.http_version, self.headers, self.body)
    }
}
