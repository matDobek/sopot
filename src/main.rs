use std::net::{TcpListener, TcpStream, SocketAddr};
use std::{thread, time};

fn handle_client(stream: TcpStream) {
    //stream.local_addr()
    match stream.peer_addr() {
        Ok(socket_addr) => {
            println!("Peer ip: {}, port: {}", socket_addr.ip(), socket_addr.port());
        },
        Err(e) => {
            println!("Obtaining address failed!");
        },
    }
}

fn main() {
    println!("Starting Sopot server...");

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

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

    drop(listener);
}
