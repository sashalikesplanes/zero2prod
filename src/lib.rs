use actix_web::{web, App, HttpResponse, HttpServer, Responder, dev::Server};

pub const URL: &str = "127.0.0.1:8080";

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn setup_server() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // Order in which routes are registered matters!
        App::new().route("/health_check", web::get().to(health_check))
    })
    .bind(URL)?
    .run();

    println!("Server listening on {URL}");

    Ok(server)
}
