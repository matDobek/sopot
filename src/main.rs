use std::net::{TcpListener, TcpStream, SocketAddrV4, Ipv4Addr};
use std::{thread, time};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    println!("Server ip: {}, port: {}", local_addr.ip(), local_addr.port());
    println!("Peer ip: {}, port: {}", peer_addr.ip(), peer_addr.port());

    // UTF-8 only?
    let mut buffer: Vec<u8> = vec![0; 1024*16]; //16 kB
    stream.read(&mut buffer);

    let request = String::from_utf8(buffer).unwrap();
    println!("Received msg: {:?}", request);
    println!("Received msg:");
    println!("--- --- ---");
    println!("{}", request);
    println!("--- --- ---");
}


fn main() {
    let host: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    let port: u16 = 3000;
    let socket_addr = SocketAddrV4::new(host, port);

    let listener = TcpListener::bind(socket_addr).unwrap();

    println!("Started Sopot server on {}", socket_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("Connection failed!");
            },
        }
    }
}
