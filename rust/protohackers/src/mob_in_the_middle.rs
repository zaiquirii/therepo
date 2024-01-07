use std::net::ToSocketAddrs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use anyhow::Result;

pub async fn run<A: tokio::net::ToSocketAddrs>(addr: A) -> Result<()> {
    let mut listener = TcpListener::bind(addr).await?;
    tracing::info!("listening for connections");
    loop {
        let (socket, socket_addr) = listener.accept().await?;
        tracing::info!("Received new connection: {:?}", socket_addr);
        tokio::spawn(async {
            match proxy_connection(socket).await {
                Ok(_) => tracing::info!("connection closed"),
                Err(e) => tracing::error!("connection closed with error: {}", e),
            }
        });
    }
}

async fn proxy_connection(stream: TcpStream) -> Result<()> {
    let mut client_stream = BufStream::new(stream);

    let mut a = "chat.protohackers.com:16963".to_socket_addrs()?;
    // let addrs = a.collect::<Vec<_>>();
    let upstream = TcpSocket::new_v4()?.connect(a.next().unwrap()).await?;
    let mut server_stream = BufStream::new(upstream);

    let mut client_buf = Vec::new();
    let mut server_buf = Vec::new();
    loop {
        tokio::select! {
            n = client_stream.read_until(b'\n', &mut client_buf) => {
                if client_buf.is_empty() {
                    break;
                }
                let msg = String::from_utf8_lossy(&client_buf);
                let msg = String::from(msg);
                tracing::info!("client msg: {}", msg);
                let modified_msg = rewrite_msg(msg);
                server_stream.write_all(modified_msg.as_bytes()).await?;
                server_stream.flush().await?;
                client_buf.clear();
            }
            n = server_stream.read_until(b'\n', &mut server_buf) => {
                if server_buf.is_empty() {
                    break;
                }
                let msg = String::from_utf8_lossy(&server_buf);
                let msg = String::from(msg);
                tracing::info!("server msg: {}", msg);
                let modified_msg = rewrite_msg(msg);
                client_stream.write_all(modified_msg.as_bytes()).await?;
                client_stream.flush().await?;
                server_buf.clear();
            }
        }
    }
    Ok(())
}

fn rewrite_msg(msg: String) -> String {
    let addrs = msg.split_ascii_whitespace()
        .filter(|s| is_boguscoin_addr(*s))
        .collect::<Vec<_>>();
    let mut ret = msg.clone();
    for a in addrs {
        ret = ret.replace(a, "7YWHMfk9JZe0LM0g1ZauHuiSxhI");
    }
    ret
}

fn is_boguscoin_addr(addr: &str) -> bool {
    let addr = addr.trim();
    if !addr.starts_with("7") {
        return false;
    }
    if addr.len() < 26 || addr.len() > 35 {
        return false;
    }
    addr.chars().all(|c| c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod test {
    use crate::mob_in_the_middle::{is_boguscoin_addr, rewrite_msg};

    #[test]
    fn bogus_coin_works() {
        assert!(is_boguscoin_addr("7adNeSwJkMakpEcln9HEtthSRtxdmEHOT8T"));
        assert!(is_boguscoin_addr("7LOrwbDlS8NujgjddyogWgIM93MV5N2VR"));
        assert!(!is_boguscoin_addr("6LOrwbDlS8NujgjddyogWgIM93MV5N2VR"));
        assert!(!is_boguscoin_addr("7L3MV5N2VR"));
        assert!(!is_boguscoin_addr("7LOrwbDlS8NujgjddyogWgIM93&MV5N2VR"));
        assert!(!is_boguscoin_addr("7LOrwbDlS8NujgjddyogWgIM93-MV5N2VRaaaaaaaaaaaaaaaaaaaaaaaaaaaaffffff"));
    }

    #[test]
    fn rewrite_msg_works() {
        assert_eq!(
            rewrite_msg("[hello] i am a coin 7LOrwbDlS8NujgjddyogWgIM93MV5N2VR".into()),
            "[hello] i am a coin 7YWHMfk9JZe0LM0g1ZauHuiSxhI"
        )
    }
}