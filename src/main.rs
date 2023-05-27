use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Attach the env logger to the `log` facade
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read config.");
    let pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres");
    let listener = TcpListener::bind((configuration.app.host, configuration.app.port))
        .expect("Listener should have bound to {PORT}");
    log::info!("Starting up");
    startup::setup_server(listener, pool)?.await
}
