use std::fmt;
use std::collections::HashMap;

pub struct Request<'a> {
    raw: &'a str,
    method: String,
    uri: String,
    http_version: String,
    headers: HashMap<String, String>,
    body: String,
}

pub fn new_request<'a>(stringified_request: &'a str) -> Request<'a> {
    let raw = &stringified_request;

    let (request_header, request_body) = {
        let request_vec: Vec<&str> = raw.split("\r\n\r\n").collect();
        match request_vec.len() {
            0 => {
                ("", "".to_string())
            },
            1 => {
                let request_header = request_vec[0];

                (request_header, "".to_string())
            },
            _ => {
                let request_header = request_vec[0];
                let request_body = request_vec[1..request_vec.len()].join("\r\n\r\n");

                (request_header, request_body)
            },
        }
    };

    let (method, uri, http_version, headers) = {
        let mut request_header_vec: Vec<&str> = request_header.split("\r\n").collect();
        let status_line: Vec<&str> = request_header_vec[0].split(" ").collect();
        let method = status_line[0].to_string();
        let uri = status_line[1].to_string();
        let http_version = status_line[2].to_string();

        let mut hsh: HashMap<String, String> = HashMap::new();
        for header in request_header_vec[1..request_header_vec.len()].iter() {
            let header_vec: Vec<&str> = header.split(": ").collect();

            hsh.insert(header_vec[0].to_string(),
                       header_vec[1..header_vec.len()].join(": "));
        }

        (method, uri, http_version, hsh)
    };

    Request { raw: raw, method: method, uri: uri, http_version: http_version, headers: headers, body: request_body }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method:\n{}\nuri:\n{}\nhttp_version:\n{}\nheaders:\n{:?}\nbody:\n{}\n",
               self.method, self.uri, self.http_version, self.headers, self.body)
    }
}
