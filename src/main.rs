use std::net::{TcpListener, TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr};
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

    drop(listener);
}
