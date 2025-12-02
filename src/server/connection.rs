use crate::server::{BroadcastEvent, ClientInfo, ServerResult};
use log::{error, info};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_connection(
    stream: TcpStream,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    broadcaster: Sender<BroadcastEvent>,
) -> ServerResult<()> {
    thread::spawn(move || {
        if let Err(err) = handle_client(stream, clients, broadcaster) {
            error!("Error while handling client: {err}");
        }
    });

    Ok(())
}

fn handle_client(
    mut stream: TcpStream,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    broadcaster: Sender<BroadcastEvent>,
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

    let username = sanitize_username(username, peer_addr);

    {
        let mut guard = clients
            .lock()
            .map_err(|err| Error::new(ErrorKind::Other, format!("clients lock poisoned: {err}")))?;
        guard.push(ClientInfo {
            username: username.clone(),
            addr: peer_addr,
            stream: stream.try_clone()?,
        });
    }

    info!("{username} joined from {peer_addr}");
    broadcaster.send(BroadcastEvent::System(format!(
        "{username} joined the chat"
    )))?;

    loop {
        let mut message = String::new();
        let bytes = reader.read_line(&mut message)?;

        if bytes == 0 {
            break;
        }

        let trimmed = message.trim();
        if trimmed.is_empty() {
            continue;
        }

        broadcaster.send(BroadcastEvent::User {
            username: username.clone(),
            content: trimmed.to_string(),
        })?;
    }

    remove_client(&clients, peer_addr)?;
    info!("{username} disconnected from {peer_addr}");
    broadcaster.send(BroadcastEvent::System(format!("{username} left the chat")))?;

    Ok(())
}

fn sanitize_username(raw: String, peer: SocketAddr) -> String {
    let cleaned = raw.trim();
    if cleaned.is_empty() {
        return format!("user-{}", peer.port());
    }

    cleaned.to_string()
}

fn remove_client(clients: &Arc<Mutex<Vec<ClientInfo>>>, addr: SocketAddr) -> ServerResult<()> {
    let mut guard = clients
        .lock()
        .map_err(|err| Error::new(ErrorKind::Other, format!("clients lock poisoned: {err}")))?;
    guard.retain(|client| client.addr != addr);
    Ok(())
}
