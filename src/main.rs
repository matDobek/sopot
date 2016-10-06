use std::{str, thread};
use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
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

fn stringify_request(buffer: &[u8]) -> String {
    let str_request = str::from_utf8(buffer).unwrap_or("");
    str_request.replace("\u{0}", "")
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
    let mut buffer: [u8; 1024*16]  = [0; 1024*16]; //16 kB

    let _no_of_bytes_read = stream.read(&mut buffer);
    let stringified_request = stringify_request(&buffer);
    //let request = request::new_request(stringified_request);
    //formatted_out!("request", request);

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
