use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::{thread};
use std::io::{Read, Write};

macro_rules! formatted_out {
    ($head:expr, $body:expr) => {
        {
            println!("----- {} -----", $head);
            println!("{}", $body);
            println!("");
        }
    };
}

fn parse_request(buffer: Vec<u8>) {
    let request = String::from_utf8(buffer).unwrap();
    let cleaned_request  = request.replace("\u{0}", "");
    let request_vec: Vec<&str> = cleaned_request.split("\r\n\r\n").collect();

    let mut request_header = "".to_string();
    let mut request_body = "".to_string();

    match request_vec.len() {
        0 => {},
        1 => {
            request_header = request_vec[0].to_string();
        },
        _ => {
            request_header = request_vec[0].to_string();
            request_body = request_vec.join("\r\n\r\n");
        },
    }

    formatted_out!("request", request);
    formatted_out!("request_header", request_header);
    formatted_out!("request_body", request_body);
}

fn handle_client(mut stream: TcpStream) {
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    println!("--- connection info ---");
    println!("Server ip: {}, port: {}", local_addr.ip(), local_addr.port());
    println!("Peer ip: {}, port: {}", peer_addr.ip(), peer_addr.port());
    println!("");

    // UTF-8 only?
    let mut buffer: Vec<u8> = vec![0; 1024*16]; //16 kB
    let _no_of_bytes_read = stream.read(&mut buffer);
    parse_request(buffer);

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
