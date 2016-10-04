use std::fmt;
use std::collections::HashMap;

pub struct Request<'a> {
    method: &'a str,
    uri: &'a str,
    http_version: &'a str,
    // TODO: what's better: HashMap or Array of Structures(key, value)
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

pub fn new_request(buffer: &str) -> Request {
    let (request_header, request_body) = {
        let request_vec: Vec<&str> = buffer.split("\r\n\r\n").collect();
        match request_vec.len() {
            0 => {
                ("", "")
            },
            1 => {
                let request_header = request_vec[0];

                (request_header, "")
            },
            _ => {
                let request_header = request_vec[0];
                let request_body = request_vec[1..request_vec.len()].join("\r\n\r\n").as_str();

                (request_header, request_body)
            },
        }
    };

    let (method, uri, http_version, headers) = {
        let mut request_header_vec: Vec<&str> = request_header.split("\r\n").collect();
        let status_line: Vec<&str> = request_header_vec[0].split(" ").collect();
        let method = status_line[0];
        let uri = status_line[1];
        let http_version = status_line[2];

        let mut hsh: HashMap<&str, &str> = HashMap::new();
        for header in request_header_vec[1..request_header_vec.len()].iter() {
            let header_vec: Vec<&str> = header.split(": ").collect();

            hsh.insert(header_vec[0],
                       header_vec[1..header_vec.len()].join(": ").as_str());
        }

        (method, uri, http_version, hsh)
    };

    Request { method: method, uri: uri, http_version: http_version, headers: headers, body: request_body }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method:\n{}\nuri:\n{}\nhttp_version:\n{}\nheaders:\n{:?}\nbody:\n{}\n",
               self.method, self.uri, self.http_version, self.headers, self.body)
    }
}
