use log::{error, info};
use std::env;
use std::error::Error;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:5000".to_string());
    info!("Connecting to server at {address}");

    let mut stream = TcpStream::connect(&address)?;
    println!("Connected to chat at {address}");

    let mut username = String::new();
    print!("Enter a username: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;

    if username.trim().is_empty() {
        username.push_str("guest");
    }

    stream.write_all(username.trim().as_bytes())?;
    stream.write_all(b"\n")?;

    let mut read_stream = stream.try_clone()?;
    thread::spawn(move || {
        if let Err(err) = read_messages(&mut read_stream) {
            error!("Server listener stopped: {err}");
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(content) => {
                if content.trim().is_empty() {
                    continue;
                }

                if let Err(err) = send_message(&mut stream, &content) {
                    error!("Failed to send message: {err}");
                    break;
                }
            }
            Err(err) => {
                error!("Failed to read from stdin: {err}");
                break;
            }
        }
    }

    Ok(())
}

fn send_message(stream: &mut TcpStream, message: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    stream.write_all(message.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    Ok(())
}

fn read_messages(stream: &mut TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut reader = BufReader::new(stream);
    loop {
        let mut buffer = String::new();
        let bytes = reader.read_line(&mut buffer)?;

        if bytes == 0 {
            break;
        }

        println!("{buffer}");
    }

    println!("Disconnected from server");
    Ok(())
}
