use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let script = include_str!("test_inputs/prime_time.txt");

    let mut stream = TcpStream::connect("0.0.0.0:14293").unwrap();
    for line in script.lines() {
        let parts = line.splitn(2, " ").collect::<Vec<_>>();
        match parts[0] {
            "send" => {
                stream.write_all(parts[1].as_bytes()).unwrap();
                stream.write_all("\n".as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            "read" => {
                let mut buf = vec![0; 1024];
                let n = stream.read(&mut buf).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
            }
            e => println!("UNSUPPORTED OP: {}", e)
        }
    }
}