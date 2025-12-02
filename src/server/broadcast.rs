use crate::server::{BroadcastEvent, ClientInfo, ServerResult};
use chrono::Local;
use log::error;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_broadcast_loop(
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    receiver: Receiver<BroadcastEvent>,
) {
    thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let Err(err) = handle_event(&clients, &event) {
                error!("Failed to handle broadcast event: {err}");
            }
        }
    });
}

fn handle_event(clients: &Arc<Mutex<Vec<ClientInfo>>>, event: &BroadcastEvent) -> ServerResult<()> {
    let timestamp = Local::now().format("%H:%M:%S");

    let formatted = match event {
        BroadcastEvent::User { username, content } => {
            format!("[{timestamp}] {username}: {content}")
        }
        BroadcastEvent::System(content) => format!("[{timestamp}] * {content}"),
    };

    send_to_all(&formatted, clients)
}

fn send_to_all(message: &str, clients: &Arc<Mutex<Vec<ClientInfo>>>) -> ServerResult<()> {
    let mut guard = clients.lock()?;
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
