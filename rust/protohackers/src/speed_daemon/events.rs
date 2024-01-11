use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::Sender;
use crate::speed_daemon::server::{ClientData, ClientId, Timestamp};
use crate::speed_daemon::ticket::Ticket;

#[derive(Debug)]
pub enum ClientEvt {
    NewClient(Sender<ClientId>, Sender<OutgoingEvt>),
    WantHeartbeat(u32),
    Register(ClientData),
    Plate(String, Timestamp),
    PrintTickets,
}

#[derive(Debug)]
pub enum OutgoingEvt {
    Heartbeat,
    Error(String),
    Ticket(Ticket),
}

impl OutgoingEvt {
    pub async fn send(&self, stream: &mut BufWriter<OwnedWriteHalf>) -> anyhow::Result<()> {
        match self {
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
}
