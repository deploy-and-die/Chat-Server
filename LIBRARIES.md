# Libraries and Study Guide

## Dependencies

- **chrono**: Provides timestamp formatting for chat messages in the broadcast loop.
- **env_logger**: Initializes a thread-safe logger that respects the `RUST_LOG` environment variable.
- **log**: Facade used throughout the server and client for structured logging.
- **std::sync::mpsc**: Standard library multi-producer, single-consumer channel used to move messages from reader threads to the broadcaster.
- **std::sync::{Arc, Mutex}**: Shared ownership and interior mutability to coordinate the client list across threads.
- **std::net::{TcpListener, TcpStream}**: TCP primitives for accepting connections and exchanging data.

## What to review for interviews

- How ownership, borrowing, and lifetimes work when sharing sockets across threads (e.g., `Arc<Mutex<...>>` and `try_clone`).
- Designing producer/consumer systems with channels and how backpressure or blocking behaves in `std::sync::mpsc`.
- Proper error handling in Rust without `unwrap`, including propagating errors with `?` and logging context.
- Strategies for handling client disconnects and stream failures in network servers.
- Basics of TCP socket programming in Rust, including `TcpListener::accept`, `TcpStream`, and `BufRead::read_line`.
- Concurrency patterns: when to prefer channels vs. shared state with mutexes, and how to avoid deadlocks.
- Using `log`/`env_logger` to add structured, thread-safe logging with environment-based filtering.
