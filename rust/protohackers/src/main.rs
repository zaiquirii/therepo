mod prime_time;
mod test_runner;
mod means_to_end;
mod budget_chat;

use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_str() {
        "smoke-test" => smoke_test().await,
        "prime-time" => prime_time::run().await,
        "means-to-end" => means_to_end::run("0.0.0.0:8000").await.unwrap(),
        "budget-chat" => budget_chat::run("0.0.0.0:8000").await.unwrap(),
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