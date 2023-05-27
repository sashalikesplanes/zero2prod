use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Attach the env logger to the `log` facade
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read config.");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind(("127.0.0.1", configuration.app_port))
        .expect("Listener should have bound to {PORT}");
    startup::setup_server(listener, pool)?.await
}
