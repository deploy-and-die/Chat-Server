mod server;

use log::info;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let port = match env::args().nth(1) {
        Some(value) => value,
        None => "5000".to_string(),
    };

    let address = format!("0.0.0.0:{}", port);

    info!("Starting server on {address}");
    server::start_server(&address)?;

    Ok(())
}
