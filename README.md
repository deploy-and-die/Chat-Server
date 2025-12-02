# Chat-Server

A multithreaded TCP chat server and client written in Rust. The server shares messages between all connected clients, tags chat lines with timestamps, and keeps connections coordinated through channels and mutex-protected state.

## Features

- Multiple simultaneous TCP clients
- Per-client threads for receiving messages
- Channel-driven broadcast loop with timestamps
- Username system announced on join/leave
- Graceful handling of disconnects and write failures
- Companion CLI client for quick testing

## Running the server

```bash
cargo run --bin Chat-Server -- 5001
```

The server listens on `0.0.0.0:<PORT>`. If no port is provided it defaults to `5001`.

## Running the client

Open another terminal and run:

```bash
cargo run --bin client -- 127.0.0.1:5001
```

Enter a username when prompted, then start chatting. Open multiple clients to see broadcasts.

## How it works

- `src/server/connection.rs` spawns a thread for each client, reads incoming lines, and sends them through an mpsc channel.
- `src/server/broadcast.rs` listens on the channel, formats messages with timestamps, and sends them to every connected stream, removing dead connections safely.
- Shared client streams are stored in `Arc<Mutex<Vec<ClientInfo>>>` so each thread can access the current peer list without data races.
- Logging uses `log` with `env_logger`, so set `RUST_LOG=info` for verbose output.
