use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};
use anyhow::{format_err, Result};

pub async fn run<A: ToSocketAddrs>(addr: A) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let room = Mutex::new(ChatRoom::new());
    let room = Arc::new(room);
    loop {
        let (socket, socket_addr) = listener.accept().await?;
        tracing::info!("Received new connection: {:?}", socket_addr);
        let h = tokio::spawn(handle_user_session(socket, room.clone()));
        tokio::spawn(async {
            if let Err(e) = h.await {
                tracing::error!("whoops {:?}", e);
            }
        });
    }
}

async fn handle_user_session(stream: TcpStream, room: Arc<Mutex<ChatRoom>>) -> Result<()> {
    let mut stream = BufStream::new(stream);
    let username = request_name(&mut stream).await?;
    let (user_id, mut send, recv) = {
        let mut r = room.lock().unwrap();
        r.join(username.clone())?
    };
    tracing::info!("user entered: {} {}", username, user_id);

    let _ = join_room(stream, &username, user_id, &mut send, recv).await;
    send.send(Msg {
        data: Arc::new(format!("* {} has left the room\n", username)),
        addr: Addr::From(user_id),
    })?;
    {
        room.lock().unwrap().end_session(user_id);
    }
    Ok(())
}

async fn join_room(mut stream: BufStream<TcpStream>,
                   username: &str,
                   user_id: UserId,
                   send: &mut Send,
                   mut recv: Recv) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        tokio::select! {
            n = stream.read_until(b'\n', &mut buf) => {
                let bytes_read = n?;
                if buf.is_empty() {
                    break Ok(());
                }
                tracing::info!("bytes read: {}", bytes_read);
                let raw_txt = String::from_utf8_lossy(buf.as_slice());
                let txt = format!("[{}] {}", username, raw_txt);
                send.send(Msg {
                    data: Arc::new(txt),
                    addr: Addr::From(user_id)
                })?;
                buf.clear();
            }
            res = recv.recv() => {
                let msg = res?;
                if msg.addr.should_receive(user_id) {
                    tracing::info!("writing message: {:?}", msg);
                    stream.write_all(msg.data.as_bytes()).await?;
                    stream.flush().await?;
                }
            }
        }
    }
}

async fn request_name(stream: &mut BufStream<TcpStream>) -> Result<String> {
    stream.write_all("Welcome to budget chat! What shall I call you?\n".as_bytes()).await?;
    stream.flush().await?;
    let mut buf = String::new();
    stream.read_line(&mut buf).await?;
    validate_username(buf.trim())
}

fn validate_username(txt: &str) -> Result<String> {
    let short = txt.trim();
    if short.len() >= 1 && short.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(short.into())
    } else {
        tracing::error!("Received invalid username: {}", short);
        Err(format_err!("invalid username: {}", short))
    }
}

type UserId = u32;
type Send = tokio::sync::broadcast::Sender<Msg>;
type Recv = tokio::sync::broadcast::Receiver<Msg>;

#[derive(Debug, Clone)]
enum Addr {
    From(UserId),
    To(UserId),
}

impl Addr {
    fn should_receive(&self, user_id: UserId) -> bool {
        match self {
            Addr::From(id) => *id != user_id,
            Addr::To(id) => *id == user_id,
        }
    }
}

#[derive(Debug, Clone)]
struct Msg {
    data: Arc<String>,
    addr: Addr,
}

struct UserInfo {
    next_id: u32,
    users: HashMap<UserId, String>,
}

struct ChatRoom {
    user_info: Mutex<UserInfo>,
    send: Send,
}

impl ChatRoom {
    fn new() -> Self {
        let (send, _) = tokio::sync::broadcast::channel(16);
        Self {
            user_info: Mutex::new(UserInfo {
                next_id: 0,
                users: HashMap::new(),
            }),
            send,
        }
    }

    fn join(&mut self, name: String) -> Result<(UserId, Send, Recv)> {
        let (user_id, msg) = {
            let mut info = self.user_info.lock().unwrap();

            let names = info.users.values()
                .map(|s| s.as_str())
                .collect::<Vec<_>>();
            let name_chunk = names.join(", ");
            let txt = format!("* The room contains: {}\n", name_chunk);

            let user_id = info.next_id;
            info.next_id += 1;
            info.users.insert(user_id, name.clone());

            (user_id, Msg {
                data: Arc::new(txt.into()),
                addr: Addr::To(user_id),
            })
        };

        let broadcast_msg = Msg {
            data: Arc::new(format!("* {} has entered the room\n", name)),
            addr: Addr::From(user_id),
        };

        let ret = (user_id, self.send.clone(), self.send.subscribe());
        ret.1.send(msg)?;
        ret.1.send(broadcast_msg)?;
        Ok(ret)
    }

    fn end_session(&mut self, user_id: UserId) {
        let mut info = self.user_info.lock().unwrap();
        info.users.remove(&user_id);
    }
}