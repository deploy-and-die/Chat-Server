pub mod broadcast;
pub mod connection;

use crate::server::broadcast::launch_broadcast_dispatcher;
use crate::server::connection::spawn_client_handler;
use log::{error, info};
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

pub type ServerResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct ClientRecord {
    pub username: String,
    pub addr: SocketAddr,
    pub stream: TcpStream,
}

#[derive(Debug, Clone)]
pub enum BroadcastMessage {
    Chat { username: String, content: String },
    Notice(String),
}

pub fn run(address: &str) -> ServerResult<()> {
    let listener = TcpListener::bind(address)?;
    info!("Listening for new clients on {address}");

    let clients: Arc<Mutex<Vec<ClientRecord>>> = Arc::new(Mutex::new(Vec::new()));
    let (tx, rx) = mpsc::channel::<BroadcastMessage>();

    launch_broadcast_dispatcher(clients.clone(), rx);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => accept_client(stream, &clients, &tx)?,
            Err(err) => error!("Failed to accept TCP connection: {err}"),
        }
    }

    Ok(())
}

fn accept_client(
    stream: TcpStream,
    clients: &Arc<Mutex<Vec<ClientRecord>>>,
    tx: &Sender<BroadcastMessage>,
) -> ServerResult<()> {
    info!("Client connected from {}", stream.peer_addr()?);
    spawn_client_handler(stream, clients.clone(), tx.clone())?;
    Ok(())
}
