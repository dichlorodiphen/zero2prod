//! src/main.rs

use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration =
        get_configuration().expect("Failed to read configuration from configuration.yaml.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener)?.await
}
