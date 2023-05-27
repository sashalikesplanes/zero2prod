use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn setup_server(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap it in a reference counter
    let connection = web::Data::new(pool);
    let server = HttpServer::new(move || {
        // Order in which routes are registered matters!
        App::new()
            .wrap(Logger::default())
            .service(health_check)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    println!("Server listening");

    Ok(server)
}
