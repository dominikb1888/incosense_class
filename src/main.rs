use sqlx::PgPool;
use std::net::TcpListener;

use incosense_class::configuration::get_configuration;
use incosense_class::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("0.0.0.0:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // TODO: Add the DB connection to our call of run using configuration
    run(listener, connection_pool)?.await
}
