use nitro::configuration::get_configuration;
use nitro::startup::run;
use nitro::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // setup telemetry
    let subscriber = get_subscriber("nitro".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // fetch app configs
    let config = get_configuration().expect("Failed to get app configuration");
    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
