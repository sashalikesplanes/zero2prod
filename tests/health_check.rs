use zero2prod::URL;

#[tokio::test]
async fn health_check_works() {
    // Setup
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(String::from("http://") + URL + "/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::setup_server().expect("Server should have bound to {URL}");
    let _ = tokio::spawn(server);
}
