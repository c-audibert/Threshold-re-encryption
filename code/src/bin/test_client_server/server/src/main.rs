use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn connect_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("mesage recu {}", String::from_utf8_lossy(&buffer));
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("erreur");
    println!("coute 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("machine {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    connect_client(stream);
                });
            }
            Err(e) => {
                eprintln!("err{}", e);
            }
        }
    }
}
