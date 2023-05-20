use zero2prod::setup_server;

// TODO get these from env
const URL: &str = "127.0.0.1";
const PORT: u16 = 8080;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    setup_server((URL, PORT))?.await
}
