mod server;

use log::info;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let port = match env::args().nth(1) {
        Some(value) => value,
        None => "5001".to_string(),
    };

    let address = format!("0.0.0.0:{}", port);

    info!("Booting chat server on {address}");
    server::run(&address)?;

    Ok(())
}
