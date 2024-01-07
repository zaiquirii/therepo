use std::collections::HashMap;
use std::net::{ToSocketAddrs, UdpSocket};
use anyhow::Result;

pub async fn run<A: ToSocketAddrs>(addr: A) -> Result<()> {
    tracing::info!("starting UdpDB");
    let socket = UdpSocket::bind(addr)?;
    let mut cache = HashMap::new();
    loop {
        let mut buf = vec![0; 1024];
        let (n, origin) = socket.recv_from(&mut buf)?;
        let cmd = Cmd::parse(&String::from_utf8_lossy(&buf[..n]));
        tracing::info!("new cmd: {:?}", cmd);

        let resp = match cmd {
            Cmd::Version => Some("version=udpdb_1.0.0"),
            Cmd::Insert { key, value } => {
                cache.insert(key, value);
                None
            }
            Cmd::Retrieve { key } => {
                cache.get(&key).map(|s| s.as_str())
            }
        };

        if let Some(r) = resp {
            socket.send_to(r.as_bytes(), origin)?;
        }
    }
}

#[derive(Debug)]
enum Cmd {
    Version,
    Insert { key: String, value: String },
    Retrieve { key: String },
}

impl Cmd {
    fn parse(txt: &str) -> Self {
        if txt == "version" {
            return Cmd::Version;
        }

        if let Some((key, value)) = txt.split_once("=") {
            return Cmd::Insert { key: key.into(), value: value.into() };
        }

        return Cmd::Retrieve { key: txt.into() };
    }
}

struct UdpDb {}