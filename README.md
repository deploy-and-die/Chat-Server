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

