pub mod broadcast;
pub mod connection;

use crate::server::broadcast::start_broadcast_loop;
use crate::server::connection::spawn_connection;
use log::{error, info};
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

pub type ServerResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct ClientInfo {
    pub username: String,
    pub addr: SocketAddr,
    pub stream: TcpStream,
}

#[derive(Debug, Clone)]
pub enum BroadcastEvent {
    User { username: String, content: String },
    System(String),
}

pub fn start_server(address: &str) -> ServerResult<()> {
    let listener = TcpListener::bind(address)?;
    info!("Server listening on {address}");

    let clients: Arc<Mutex<Vec<ClientInfo>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel::<BroadcastEvent>();

    start_broadcast_loop(clients.clone(), rx);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => handle_new_connection(stream, &clients, &tx)?,
            Err(err) => error!("Failed to accept connection: {err}"),
        }
    }

    Ok(())
}

fn handle_new_connection(
    stream: TcpStream,
    clients: &Arc<Mutex<Vec<ClientInfo>>>,
    tx: &Sender<BroadcastEvent>,
) -> ServerResult<()> {
    info!("Client connected: {}", stream.peer_addr()?);
    spawn_connection(stream, clients.clone(), tx.clone())?;
    Ok(())
}
