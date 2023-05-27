use reqwest::StatusCode;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup,
};

const URL: &str = "127.0.0.1";

#[tokio::test]
async fn health_check_works() -> std::io::Result<()> {
    // Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(app.address + "/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() -> std::io::Result<()> {
    // Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=the%20sasha&email=sasha%40email.com";
    let response = client
        .post(app.address + "/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "the sasha");
    assert_eq!(saved.email, "sasha@email.com");
    Ok(())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Setup
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=sasha", "missing the email"),
        ("email=sasha%40email.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, message) in test_cases {
        // Act
        let response = client
            .post(app.address.clone() + "/subscribe")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            message
        )
    }
}

struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

/// Spin up an instance of the application at a random port
/// And return its address
async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().expect("Failed to read config");
    configuration.database.name = uuid::Uuid::new_v4().to_string();
    // Create a bound listener
    let listener = TcpListener::bind((URL, 0)).expect("Listener should have bouund to random port");
    let address = &listener
        .local_addr()
        .expect("Listener should have an address");
    let address = format!("http://{}:{}", address.ip(), address.port());
    let pool = configure_database(&configuration.database).await;

    let server =
        startup::setup_server(listener, pool.clone()).expect("Server should have bound to {URL}");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Make a temp connection to create the db
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to db");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create DB");

    // Establish the proper DB connection
    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to PSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migartions failed to run");
    pool
}
