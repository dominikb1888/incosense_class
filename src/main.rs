use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

use incosense_class::configuration::get_configuration;
use incosense_class::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let connection = PgConnection::connect("")
        .await
        .expect("Failed to connect to Postgres.");
    // TODO: Add the DB connection to our call of run using configuration
    run(listener, connection)?.await
}
