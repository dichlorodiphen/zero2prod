//! src/main.rs

use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration =
        get_configuration().expect("Failed to read configuration from configuration.yaml.");
    let pool_connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    startup::run(listener, pool_connection)?.await
}
