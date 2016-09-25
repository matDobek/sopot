use std::net::{TcpListener, TcpStream};
use std::{thread, time};

fn handle_client(stream: TcpStream) {
    println!("Connection Success!");
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
