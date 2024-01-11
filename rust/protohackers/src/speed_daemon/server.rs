use std::collections::HashMap;
use crate::speed_daemon::events::OutgoingEvt;
use tokio::sync::mpsc::{Sender, UnboundedReceiver};
use tokio::time;
use crate::speed_daemon::events::ClientEvt;
use crate::speed_daemon::ticket::Ticket;

pub async fn run_server(mut server_rx: UnboundedReceiver<ClientIdEvt>) -> anyhow::Result<()> {
    let mut server = Server::new();
    loop {
        let evt = server_rx.recv().await;
        if evt.is_none() {
            break;
        }
        let (client_id, evt) = evt.unwrap();
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
            ClientEvt::PrintTickets => {
                tracing::info!("Pending tickts: ");
                for t in &server.pending_tickets {
                    tracing::info!("{:?}", t);
                }
            }
        }
    }
    Ok(())
}

pub type ClientIdEvt = (ClientId, ClientEvt);
pub type Road = u16;
pub type Mile = u16;
pub type Timestamp = u32;

pub type ClientId = u32;

#[derive(Debug)]
pub enum ClientData {
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

    async fn identify_client(&mut self, client_id: ClientId, data: ClientData) -> anyhow::Result<()> {
        let client = self.clients.get_mut(&client_id).unwrap();
        if client.data.is_some() {
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

    async fn record_plate(&mut self, client_id: ClientId, plate: String, timestamp: Timestamp) -> anyhow::Result<()> {
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

    async fn client_err(&mut self, client_id: ClientId, err: String) -> anyhow::Result<()> {
        let client = self.clients.get(&client_id).unwrap();
        client.tx.send(OutgoingEvt::Error(err)).await?;
        self.clients.remove(&client_id);
        Ok(())
    }

    async fn send_ticket(&mut self, ticket: Ticket) -> anyhow::Result<()> {
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

#[derive(Default)]
struct PlateData {
    tickets: Vec<u32>,
    data: HashMap<Road, Vec<(Mile, Timestamp)>>,
}

struct PlateTracker {
    data: HashMap<String, PlateData>,
}

impl PlateTracker {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn record_plate(&mut self, plate: String, road: Road, mile: Mile, timestamp: Timestamp, limit: u16) -> Vec<Ticket> {
        let mut results = Vec::new();
        let plate_data = self.data.entry(plate.clone()).or_insert_with(PlateData::default);
        let road_data = plate_data.data.entry(road).or_insert_with(Vec::default);
        road_data.push((mile, timestamp));
        road_data.sort_by_key(|t| t.1);

        tracing::info!("Recording data: {:?}", road_data);
        if road_data.len() < 2 {
            return results;
        }

        for i in 0..road_data.len() - 1 {
            let pent = road_data[i];
            let last = road_data[i + 1];

            let dist = last.0.abs_diff(pent.0) as f64;
            let time = (last.1 - pent.1) as f64;
            let m_per_s = dist / time;
            let m_per_h = m_per_s * 60.0 * 60.0;
            if m_per_h as u16 > limit {
                let day_1 = pent.1 / 86400;
                let day_2 = last.1 / 86400;

                if plate_data.tickets.iter().any(|t| *t == day_1 || *t == day_2) {
                    tracing::info!("ticket already issued for day");
                    continue;
                }

                plate_data.tickets.push(day_1);
                if day_1 != day_2 {
                    plate_data.tickets.push(day_2);
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
                tracing::info!("TICKET {:?}", ticket);
                results.push(ticket);
            }
        }
        results
    }
}
