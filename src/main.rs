use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::{thread};
use std::io::{Read, Write, BufReader, BufRead};
use std::collections::HashMap;

pub use request::{Request};

mod request;

macro_rules! formatted_out {
    ($head:expr, $body:expr) => {
        {
            println!("----- {} -----", $head);
            println!("{}", $body);
            println!("");
        }
    };
}

fn stringify_request(buffer: Vec<u8>) -> String {
    let request = String::from_utf8(buffer).unwrap();
    request.replace("\u{0}", "")
}

fn parse_request(request: String) {
    let request_vec: Vec<&str> = request.split("\r\n\r\n").collect();

    let mut request_header = "".to_string();
    let mut request_body = "".to_string();

    match request_vec.len() {
        0 => {},
        1 => {
            request_header = request_vec[0].to_string();
        },
        _ => {
            request_header = request_vec[0].to_string();
            request_body = request_vec[1..request_vec.len()].join("\r\n\r\n");
        },
    }
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
                       header_vec[1..header_vec.len()].join(": ").to_string());
        }

        (method, uri, http_version, hsh)
    };

    let request = Request { method: method, uri: uri, http_version: http_version, headers: headers, body: request_body };
    formatted_out!("request", request);
}

fn handle_client(mut stream: TcpStream) {
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    println!("--- connection info ---");
    println!("Server ip: {}, port: {}", local_addr.ip(), local_addr.port());
    println!("Peer ip: {}, port: {}", peer_addr.ip(), peer_addr.port());
    println!("");

    // TODO: what's the memory usage of buffer?
    // TODO: use BufReader
    // TODO: it's UTF-8 only?
    let mut buffer: Vec<u8> = vec![0; 1024*16]; //16 kB

    let _no_of_bytes_read = stream.read(&mut buffer);
    let stringified_request = stringify_request(buffer);
    parse_request(stringified_request);

    let response  = String::from("HTTP/1.1 200 OK\n\nServed by Sopot");
    let _no_of_bytes_written = stream.write(response.as_bytes());
}


fn main() {
    let host: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 3000;
    let socket_addr = SocketAddrV4::new(host, port);

    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Started Sopot server on {}", socket_addr);
    println!("");

    // TODO: what's difference between #incoming and #accept?
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(_) => {
                println!("Connection failed!");
            },
        }
    }
}
