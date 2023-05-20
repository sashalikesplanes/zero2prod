use zero2prod::setup_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_server()?.await
}
