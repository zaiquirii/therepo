use std::collections::HashMap;
use std::future::Future;
use anyhow::{Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufStream};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time;

/// General idea
/// - Each socket is managed by its own task.
/// - sockets parse messages and push them into a single worker queue
/// - worker queue does the real work.
/// - Pushes back out to the sockets for delivery
///
/// Constraints:
/// - Build without a mutex, no shared state, only message passing
pub async fn run<A: ToSocketAddrs>(addr: A) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let (server_tx, server_rx) = mpsc::channel(32);
    tokio::spawn(run_server(server_rx));

    tracing::info!("Starting speed-daemon");

    let mut client_id = 0;
    loop {
        let (socket, addr) = listener.accept().await?;
        client_id += 1;
        tracing::info!("Received new connection: {:?}", addr);
        let tx = server_tx.clone();
        let id = client_id;
        tokio::spawn(async move {
            let e = handle_client_session(socket, tx).await;
            tracing::info!("Client connection closed: {}", id)
        });
    }
}

async fn handle_client_session(socket: TcpStream, server_tx: Sender<ClientIdEvt>) -> Result<()> {
    let mut stream = BufStream::new(socket);
    let (tx, mut rx) = mpsc::channel(32);
    let (id_tx, mut id_rx) = mpsc::channel(1);

    server_tx.send((0, ClientEvt::NewClient(id_tx, tx))).await?;
    let client_id = id_rx.recv().await.unwrap();

    loop {
        tokio::select! {
            res = stream.read_u8() => {
                let n = res?;
                let evt = parse_event(n, &mut stream).await?;
                server_tx.send((client_id, evt)).await?;
            }
            evt = rx.recv() =>  {
                if evt.is_none() {
                    break;
                }
                let evt = evt.unwrap();
                // tracing::info!("Sending event: {} {:?}", client_id,  evt);
                send_event(evt, &mut stream).await?;
            }
        }
    }
    Ok(())
}

async fn parse_event(evt_type: u8, stream: &mut BufStream<TcpStream>) -> Result<ClientEvt> {
    let evt = match evt_type {
        0x20 => {
            let len = stream.read_u8().await?;
            let mut buf = vec![0; len as usize];
            let _ = stream.read_exact(&mut buf).await?;
            ClientEvt::Plate(String::from_utf8(buf)?, stream.read_u32().await?)
        }
        0x40 => {
            let interval = stream.read_u32().await?;
            ClientEvt::WantHeartbeat(interval)
        }
        0x80 => {
            let road = stream.read_u16().await?;
            let mile = stream.read_u16().await?;
            let limit = stream.read_u16().await?;
            ClientEvt::Register(ClientData::Camera { road, mile, limit })
        }
        0x81 => {
            let num_roads = stream.read_u8().await?;
            let mut roads = Vec::with_capacity(num_roads as usize);
            for _ in 0..num_roads {
                roads.push(stream.read_u16().await?);
            }
            ClientEvt::Register(ClientData::Dispatcher(roads))
        }
        _ => panic!("UNSUPPORTED TYPE: 0x{:X}", evt_type)
    };
    Ok(evt)
}

async fn send_event(evt: OutgoingEvt, stream: &mut BufStream<TcpStream>) -> Result<()> {
    match evt {
        OutgoingEvt::Heartbeat => {
            stream.write_u8(0x41).await?;
        }
        OutgoingEvt::Error(msg) => {
            stream.write_u8(0x10).await?;
            stream.write_u8(msg.len() as u8).await?;
            stream.write_all(msg.as_bytes()).await?;
        }
        OutgoingEvt::Ticket(ticket) => {
            stream.write_u8(0x21).await?;
            stream.write_u8(ticket.plate.len() as u8).await?;
            stream.write_all(ticket.plate.as_bytes()).await?;
            stream.write_u16(ticket.road).await?;
            stream.write_u16(ticket.mile1).await?;
            stream.write_u32(ticket.timestamp1).await?;
            stream.write_u16(ticket.mile2).await?;
            stream.write_u32(ticket.timestamp2).await?;
            stream.write_u16(ticket.speed).await?;
        }
    }
    stream.flush().await?;
    Ok(())
}

async fn run_server(mut server_rx: Receiver<ClientIdEvt>) -> Result<()> {
    let mut server = Server::new();
    loop {
        let evt = server_rx.recv().await;
        if evt.is_none() {
            break;
        }
        let (client_id, evt) = evt.unwrap();
        // tracing::info!("Processing event: {} {:?}", client_id, evt);
        match evt {
            ClientEvt::NewClient(id_tx, client_tx) => {
                let id = server.register(client_tx);
                tracing::info!("New client registered: {}", id);
                id_tx.send(id).await?;
            }
            ClientEvt::Plate(plate, timestamp) => {
                server.record_plate(client_id, plate, timestamp).await?;
            }
            ClientEvt::WantHeartbeat(interval) => {
                if interval == 0 {
                    continue;
                }
                let tx = server.client_tx(client_id).unwrap().clone();
                tokio::spawn(async move {
                    let mut interval = time::interval(time::Duration::from_millis((interval * 100) as u64));
                    loop {
                        interval.tick().await;
                        if tx.send(OutgoingEvt::Heartbeat).await.is_err() {
                            break;
                        }
                    }
                });
            }
            ClientEvt::Register(data) => server.identify_client(client_id, data).await?,
        }
    }
    Ok(())
}

type ClientIdEvt = (ClientId, ClientEvt);
type Road = u16;
type Mile = u16;
type Timestamp = u32;

#[derive(Debug)]
enum ClientEvt {
    NewClient(Sender<ClientId>, Sender<OutgoingEvt>),
    WantHeartbeat(u32),
    Register(ClientData),
    Plate(String, Timestamp),
}

#[derive(Debug)]
enum OutgoingEvt {
    Heartbeat,
    Error(String),
    Ticket(Ticket),
}

type ClientId = u32;

#[derive(Debug)]
enum ClientData {
    Camera {
        road: u16,
        mile: u16,
        limit: u16,
    },
    Dispatcher(Vec<Road>),
}

struct Client {
    tx: Sender<OutgoingEvt>,
    data: Option<ClientData>,
}

struct Server {
    next_id: ClientId,
    clients: HashMap<ClientId, Client>,
    tracker: PlateTracker,
    pending_tickets: Vec<Ticket>,
}

impl Server {
    fn new() -> Self {
        let s = Self {
            next_id: 0,
            clients: HashMap::new(),
            tracker: PlateTracker::new(),
            pending_tickets: Vec::new(),
        };
        s
    }

    fn register(&mut self, client_tx: Sender<OutgoingEvt>) -> ClientId {
        let id = self.next_id;
        self.next_id += 1;
        self.clients.insert(id, Client { tx: client_tx, data: None });
        id
    }

    fn client_tx(&self, client_id: ClientId) -> Option<&Sender<OutgoingEvt>> {
        self.clients.get(&client_id).map(|c| &c.tx)
    }

    async fn identify_client(&mut self, client_id: ClientId, data: ClientData) -> Result<()> {
        let client = self.clients.get_mut(&client_id).unwrap();
        if client.data.is_some() {
            // client.tx.send(OutgoingEvt::Error("already registered".into())).await?;
            // self.clients.remove(&client_id);
            self.client_err(client_id, "already registered".into()).await?;
        } else {
            client.data = Some(data);
            if let Some(ClientData::Dispatcher(roads)) = &client.data {
                let mut i = 0;
                while i < self.pending_tickets.len() {
                    let ticket = &self.pending_tickets[i];
                    if roads.iter().any(|r| *r == ticket.road) {
                        let ticket = self.pending_tickets.swap_remove(i);
                        client.tx.send(OutgoingEvt::Ticket(ticket)).await?;
                    } else {
                        i += 1;
                    }
                }
            }
        }
        Ok(())
    }

    async fn record_plate(&mut self, client_id: ClientId, plate: String, timestamp: Timestamp) -> Result<()> {
        let client = self.clients.get(&client_id).unwrap();
        if let Some(ClientData::Camera { road, mile, limit }) = client.data {
            for t in self.tracker.record_plate(plate, road, mile, timestamp, limit) {
                self.send_ticket(t).await?;
            }
        } else {
            self.client_err(client_id, "client not identified as camera".into()).await?;
        }
        Ok(())
    }

    async fn client_err(&mut self, client_id: ClientId, err: String) -> Result<()> {
        let client = self.clients.get(&client_id).unwrap();
        client.tx.send(OutgoingEvt::Error(err)).await?;
        self.clients.remove(&client_id);
        Ok(())
    }

    async fn send_ticket(&mut self, ticket: Ticket) -> Result<()> {
        // tracing::info!("Sending ticket: {:?}", ticket);

        for c in self.clients.values() {
            if let Some(ClientData::Dispatcher(roads)) = &c.data {
                if roads.iter().any(|r| *r == ticket.road) {
                    c.tx.send(OutgoingEvt::Ticket(ticket)).await?;
                    return Ok(());
                }
            }
        }
        self.pending_tickets.push(ticket);
        Ok(())
    }
}

#[derive(Debug)]
struct Ticket {
    plate: String,
    road: Road,
    mile1: Mile,
    timestamp1: u32,
    mile2: Mile,
    timestamp2: u32,
    speed: u16,
    day_1: u32,
    day_2: u32,
}

struct PlateTracker {
    data: HashMap<String, HashMap<Road, Vec<(Mile, Timestamp)>>>,
    tickets: Vec<(String, u32)>,
}

impl PlateTracker {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            tickets: Vec::new(),
        }
    }

    fn record_plate(&mut self, plate: String, road: Road, mile: Mile, timestamp: Timestamp, limit: u16) -> Vec<Ticket> {
        let mut results = Vec::new();
        let plate_data = self.data.entry(plate.clone()).or_insert_with(HashMap::new);
        let road_data = plate_data.entry(road).or_insert_with(Vec::new);
        road_data.push((mile, timestamp));
        road_data.sort_by_key(|t| t.1);

        if road_data.len() < 2 {
            return results;
        }

        let new_i = road_data.iter().rposition(|t| t.1 == timestamp).unwrap();
        let mut count = 0;
        let mut start = new_i;
        if new_i > 0 {
            count += 1;
            start -= 1;
        }
        if new_i == road_data.len() - 1 {
            count += 1;
        }

        for i in start..start + count {
            let pent = road_data[i];
            let last = road_data[i + 1];

            let dist = last.0.abs_diff(pent.0) as f64;
            let time = (last.1 - pent.1) as f64;
            let m_per_s = dist / time;
            let m_per_h = m_per_s * 60.0 * 60.0;
            if m_per_h as u16 > limit {
                let day_1 = pent.1 / 86400;
                let day_2 = last.1 / 86400;

                if self.tickets.iter().any(|t| t.0 == plate && (t.1 == day_1 || t.1 == day_2)) {
                    tracing::info!("ticket already issued for day");
                    return results;
                }

                self.tickets.push((plate.clone(), day_1));
                if day_1 != day_2 {
                    self.tickets.push((plate.clone(), day_2));
                }
                let ticket = Ticket {
                    plate: plate.clone(),
                    road,
                    mile1: pent.0,
                    timestamp1: pent.1,
                    mile2: last.0,
                    timestamp2: last.1,
                    speed: (m_per_h * 100.0) as u16,
                    day_1,
                    day_2,
                };
                results.push(ticket);
            }
        }
        results
    }
}
