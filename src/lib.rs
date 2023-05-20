use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn setup_server(
    socket_address: impl std::net::ToSocketAddrs,
) -> Result<Server, std::io::Error> {




    let server = HttpServer::new(|| {
        // Order in which routes are registered matters!
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind(socket_address)?
    .run();

    println!("Server listening");

    Ok(server)
}
