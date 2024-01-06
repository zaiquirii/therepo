use tokio::net::{TcpStream, ToSocketAddrs};
use tracing;
use anyhow::{format_err, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};

pub async fn run<A: ToSocketAddrs>(addr: A) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        let (socket, socket_addr) = listener.accept().await?;
        tracing::info!("Received new connection: {:?}", socket_addr);
        tokio::spawn(handle_socket(socket));
    }
}

async fn handle_socket(stream: TcpStream) -> Result<()> {
    let mut stream = BufStream::new(stream);
    let mut cache = Cache::new();
    let mut buf = vec![0; 9];

    loop {
        stream.read_exact(&mut buf).await?;
        let op = Operation::parse(&buf)?;
        if let Some(answer) = cache.process(op) {
            stream.write_i32(answer).await?;
            stream.flush().await?;
        }
    }
}

#[derive(Debug)]
enum Operation {
    Insert { timestamp: i32, price: i32 },
    Query { min_time: i32, max_time: i32 },
}

impl Operation {
    fn parse(data: &[u8]) -> Result<Operation> {
        let op = data[0];
        let int_0 = i32::from_be_bytes(data[1..5].try_into().unwrap());
        let int_1 = i32::from_be_bytes(data[5..9].try_into().unwrap());
        match op as char {
            'I' => Ok(Operation::Insert {
                timestamp: int_0,
                price: int_1,
            }),
            'Q' => Ok(Operation::Query {
                min_time: int_0,
                max_time: int_1,
            }),
            _ => Err(format_err!("bad message: {:?}", data))
        }
    }
}

struct DataPoint {
    timestamp: i32,
    price: i32,
}

struct Cache {
    data: Vec<DataPoint>,
    sorted: bool,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: Vec::new(),
            sorted: false,
        }
    }

    fn process(&mut self, op: Operation) -> Option<i32> {
        match op {
            Operation::Insert { timestamp, price } => {
                self.data.push(DataPoint { timestamp, price });
                self.sorted = false;
                None
            }
            Operation::Query { min_time, max_time } => {
                tracing::info!("Query start");
                if min_time > max_time {
                    return Some(0);
                }

                if !self.sorted {
                    self.data.sort_by_key(|p| p.timestamp);
                    self.sorted = true;
                }

                let mut count = 0i64;
                let mut total = 0i64;
                for x in self.data.iter()
                    .skip_while(|p| p.timestamp < min_time)
                    .take_while(|p| p.timestamp <= max_time)
                    .map(|p| p.price) {
                    count += 1;
                    total += x as i64;
                }
                if count == 0 {
                    Some(0)
                } else {
                    Some((total / count) as i32)
                }
            }
        }
    }
}