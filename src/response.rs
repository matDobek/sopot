use std::collections::HashMap;

pub struct Response {
    http_version: String,
    status_code: String,
    reason_phrase: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new() -> Response {
        let mut headers = HashMap::new();
        headers.insert("".to_string(), "".to_string());

        let mut response = Response {
            http_version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            reason_phrase: "OK".to_string(),
            headers: headers,
            body: "Served by Sopot".to_string(),
        };

        response
    }

    pub fn stringify_response(&mut self) -> String {
        let status_line = self.stringify_status_line();
        let headers = self.stringify_headers();
        let ref body = self.body;

        format!("{}\n{}\n{}", status_line, headers, body)
    }

    fn stringify_status_line(&mut self) -> String {
        format!("{} {} {}", self.http_version, self.status_code, self.reason_phrase)
    }

    fn stringify_headers(&mut self) -> String {
        let mut str = String::with_capacity(1024);
        let ref headers = self.headers;
        for (header, value) in headers {
            str.push_str(&header[..]);
            str.push_str(": ");
            str.push_str(&value[..]);
            str.push_str("\n");
        }

        str
    }
}
