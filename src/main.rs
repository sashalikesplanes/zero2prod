use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config.");
    let pool = PgPool::connect_lazy_with(configuration.database.with_db());
    let listener = TcpListener::bind((configuration.app.host, configuration.app.port))
        .expect("Listener should have bound to {PORT}");
    startup::setup_server(listener, pool)?.await
}
