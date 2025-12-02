use crate::server::{BroadcastMessage, ClientRecord, ServerResult};
use log::{error, info};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_client_handler(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<ClientRecord>>>,
    broadcaster: Sender<BroadcastMessage>,
) -> ServerResult<()> {
    thread::spawn(move || {
        if let Err(err) = run_client_session(stream, clients, broadcaster) {
            error!("Client session ended in error: {err}");
        }
    });

    Ok(())
}

fn run_client_session(
    mut stream: TcpStream,
    clients: Arc<Mutex<Vec<ClientRecord>>>,
    broadcaster: Sender<BroadcastMessage>,
) -> ServerResult<()> {
    let peer_addr = stream.peer_addr()?;

    stream.write_all(b"Enter username: ")?;
    stream.flush()?;

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut username = String::new();
    let read = reader.read_line(&mut username)?;
    if read == 0 {
        return Ok(());
    }

    let username = normalize_username(username, peer_addr);
    let client_stream = stream.try_clone()?;
    register_client(&clients, peer_addr, client_stream, &username)?;

    info!("{username} joined from {peer_addr}");
    broadcaster.send(BroadcastMessage::Notice(format!(
        "{username} joined the chat"
    )))?;

    loop {
        let mut incoming = String::new();
        let bytes = reader.read_line(&mut incoming)?;

        if bytes == 0 {
            break;
        }

        let trimmed = incoming.trim();
        if trimmed.is_empty() {
            continue;
        }

        broadcaster.send(BroadcastMessage::Chat {
            username: username.clone(),
            content: trimmed.to_string(),
        })?;
    }

    drop_client_entry(&clients, peer_addr)?;
    info!("{username} disconnected from {peer_addr}");
    broadcaster.send(BroadcastMessage::Notice(format!("{username} left the chat")))?;

    Ok(())
}

fn normalize_username(raw: String, peer: SocketAddr) -> String {
    let cleaned = raw.trim();
    if cleaned.is_empty() {
        return format!("user-{}", peer.port());
    }

    cleaned.to_string()
}

fn register_client(
    clients: &Arc<Mutex<Vec<ClientRecord>>>,
    addr: SocketAddr,
    stream: TcpStream,
    username: &str,
) -> ServerResult<()> {
    let mut guard = clients.lock().map_err(|err| {
        io::Error::new(io::ErrorKind::Other, format!("Clients lock poisoned: {err}"))
    })?;

    guard.push(ClientRecord {
        username: username.to_string(),
        addr,
        stream,
    });

    Ok(())
}

fn drop_client_entry(clients: &Arc<Mutex<Vec<ClientRecord>>>, addr: SocketAddr) -> ServerResult<()> {
    let mut guard = clients.lock().map_err(|err| {
        io::Error::new(io::ErrorKind::Other, format!("Clients lock poisoned: {err}"))
    })?;
    guard.retain(|client| client.addr != addr);
    Ok(())
}
