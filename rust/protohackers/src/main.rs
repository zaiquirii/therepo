mod prime_time;
mod means_to_end;
mod budget_chat;
mod udpdb;
mod mob_in_the_middle;

use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    let addr = &args[2];
    match cmd.as_str() {
        "smoke-test" => smoke_test().await,
        "prime-time" => prime_time::run().await,
        "means-to-end" => means_to_end::run(addr).await.unwrap(),
        "budget-chat" => budget_chat::run(addr).await.unwrap(),
        "udpdb" => udpdb::run(addr).await.unwrap(),
        "mob-in-the-middle" => mob_in_the_middle::run(addr).await.unwrap(),
        _ => println!("unsupported command"),
    }
}

async fn smoke_test() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:14293").await.unwrap();
    println!("listening");
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("Received new connection");
        tokio::spawn(async move {
            loop {
                let mut buf = vec![0; 1024];
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");
                if n == 0 {
                    println!("connection closed");
                    return;
                }
                socket
                    .write_all(&buf[..n])
                    .await
                    .expect("failed to write data to socket")
            }
        });
    }
}