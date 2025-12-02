use crate::server::{BroadcastMessage, ClientRecord, ServerResult};
use chrono::Local;
use log::error;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn launch_broadcast_dispatcher(
    clients: Arc<Mutex<Vec<ClientRecord>>>,
    receiver: Receiver<BroadcastMessage>,
) {
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let Err(err) = handle_message(&clients, &event) {
                error!("Broadcast dispatch failed: {err}");
            }
        }
    });
}

fn handle_message(
    clients: &Arc<Mutex<Vec<ClientRecord>>>,
    event: &BroadcastMessage,
) -> ServerResult<()> {
    let timestamp = Local::now().format("%H:%M:%S");

    let formatted = match event {
        BroadcastMessage::Chat { username, content } => {
            format!("[{timestamp}] {username}: {content}")
        }
        BroadcastMessage::Notice(content) => format!("[{timestamp}] * {content}"),
    };

    broadcast_line(&formatted, clients)
}

fn broadcast_line(message: &str, clients: &Arc<Mutex<Vec<ClientRecord>>>) -> ServerResult<()> {
    let mut guard = clients.lock().map_err(|err| {
        io::Error::new(io::ErrorKind::Other, format!("Clients lock poisoned: {err}"))
    })?;
    let mut failed_indices = Vec::new();

    for (index, client) in guard.iter_mut().enumerate() {
        if let Err(err) = writeln!(client.stream, "{message}") {
            error!(
                "Failed to send message to {} ({}): {err}",
                client.username, client.addr
            );
            failed_indices.push(index);
            continue;
        }

        if let Err(err) = client.stream.flush() {
            error!(
                "Failed to flush stream for {} ({}): {err}",
                client.username, client.addr
            );
            failed_indices.push(index);
        }
    }

    for index in failed_indices.into_iter().rev() {
        guard.remove(index);
    }

    Ok(())
}
