use tokio::io::{AsyncBufReadExt,AsyncWriteExt, BufStream};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize)]
struct Request {
    method: String,
    number: Number,
}

#[derive(Serialize)]
struct Response {
    method: String,
    prime: bool,
}

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:14293").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("Received new connection");

        tokio::spawn(async move {
            let mut buf_stream = BufStream::new(socket);
            loop {
                // let mut buf = Vec::new();
                let mut buf = String::new();
                let n = buf_stream.read_line(&mut buf).await.unwrap();
                // let data_str = String::from_utf8_lossy(&buf[..n]);
                // let trimmed = data_str.trim();
                let trimmed = buf.trim();

                if n == 0 {
                    buf_stream.write_all("malformed".as_bytes()).await.unwrap();
                    break;
                }
                match serde_json::from_str::<Request>(&trimmed) {
                    Err(e) => {
                        println!("ERROR: {:?}", e);
                        buf_stream.write_all("malformed".as_bytes()).await.unwrap();
                        break;
                    }
                    Ok(req) => match process(req) {
                        None => {
                            buf_stream.write_all("malformed".as_bytes()).await.unwrap();
                            break;
                        }
                        Some(res) => {
                            let data = serde_json::to_string(&res).unwrap();
                            println!("response: {}", data);
                            buf_stream.write_all(data.as_bytes()).await.unwrap();
                            buf_stream.write_all("\n".as_bytes()).await.unwrap();
                            buf_stream.flush().await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

fn process(req: Request) -> Option<Response> {
    if req.method != "isPrime" {
        return None;
    }

    let mut prime = false;
    if let Some(x) = req.number.as_i64() {
        prime = is_prime(x);
    }

    Some(Response {
        method: "isPrime".into(),
        prime,
    })
}

fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true
    }

    let mut div = 2;
    let cutoff = x / 2 + 2;
    while div < cutoff {
        if x % div == 0 {
            return false;
        }
        div += 1;
    }
    true
}

#[cfg(test)]
mod test {
    use crate::prime_time::is_prime;

    #[test]
    fn is_prime_works() {
        let cases = vec![
            (-100, false),
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (21, false),
            (53, true),
        ];
        for case in cases {
            assert_eq!(is_prime(case.0), case.1, "is_prime({})", case.0);
        }
    }
}