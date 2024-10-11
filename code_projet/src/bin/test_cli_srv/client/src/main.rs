use std::io::Write;
use std::net::TcpStream;

fn main() {
    //ip des machinnes à contacter la srv
    let addresses = vec!["172.19.0.2"];

    for addr in addresses {
        match TcpStream::connect(format!("{}:8080", addr)) {
            Ok(mut stream) => {
                println!("je suis connecter à {}", addr);
                    stream.write(b"ciphertext a envoyer \n");
                }
            Err(e) => {
                println!("err {}", e);
        }
    }
}}
