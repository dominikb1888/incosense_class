use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

use incosense_class::configuration::get_configuration;
use incosense_class::startup::run;
use incosense_class::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("incosense_class".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    // TODO: Add the DB connection to our call of run using configuration
    run(listener, connection_pool)?.await
}
