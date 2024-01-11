use tokio::io::{AsyncReadExt, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use crate::speed_daemon::events::ClientEvt;
use crate::speed_daemon::server::{ClientData, ClientIdEvt};

pub(crate) async fn handle_client_session(socket: TcpStream, server_tx: UnboundedSender<ClientIdEvt>) -> anyhow::Result<()> {
    let (read, write) = socket.into_split();
    let mut read_stream = BufReader::new(read);
    let mut write_stream = BufWriter::new(write);

    let (tx, mut rx) = mpsc::channel(32);
    let (id_tx, mut id_rx) = mpsc::channel(1);

    server_tx.send((0, ClientEvt::NewClient(id_tx, tx)))?;
    let client_id = id_rx.recv().await.unwrap();

    let read_handle = tokio::spawn(async move {
        loop {
            let n = read_stream.read_u8().await?;
            let evt = parse_event(n, &mut read_stream).await?;
            server_tx.send((client_id, evt))?;
        }
    });

    loop {
        let evt = rx.recv().await;
        match evt {
            None => break,
            Some(evt) => {
                evt.send(&mut write_stream).await?;
            }
        }
    }
    let _: anyhow::Result<()> = read_handle.await?;
    Ok(())
}

async fn parse_event(evt_type: u8, stream: &mut BufReader<OwnedReadHalf>) -> anyhow::Result<ClientEvt> {
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
