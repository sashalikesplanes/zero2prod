const URL: &str = "127.0.0.1";
const PORT: u16 = 8079;

#[tokio::test]
async fn health_check_works() {
    // Setup
    spawn_app();
    let client = reqwest::Client::new();
    let url = format!("http://{URL}:{PORT}/health_check");

    // Act
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::setup_server((URL, PORT)).expect("Server should have bound to {URL}");
    let _ = tokio::spawn(server);
}
