mod client_session;
mod server;
mod events;
mod ticket;

use anyhow::Result;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc;

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
    let (server_tx, server_rx) = mpsc::unbounded_channel();
    tokio::spawn(server::run_server(server_rx));
    tracing::info!("Starting speed-daemon");

    let mut client_id = 0;
    loop {
        let (socket, addr) = listener.accept().await?;
        client_id += 1;
        tracing::info!("Received new connection: {:?}", addr);
        let tx = server_tx.clone();
        let id = client_id;
        tokio::spawn(async move {
            let _ = client_session::handle_client_session(socket, tx).await;
            tracing::info!("Client connection closed: {}", id)
        });
    }
}
