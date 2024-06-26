use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configuration::{self, DatabaseSettings},
    startup,
};

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // given: our app is running
    let TestApp { address, db_pool } = spawn_app().await;

    // and: an HTTP client
    let client = reqwest::Client::new();

    // when: we send a request that's missing a name to the endpoint
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // then: we receive a 200 OK
    assert_eq!(response.status(), 200);

    // and: the new subscription exists in the database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_a_required_parameter_is_missing() {
    // given: our app is running
    let TestApp { address, .. } = spawn_app().await;

    // and: an HTTP client
    let client = reqwest::Client::new();

    // when: we send a request that's missing a required field to the endpoint
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "mising the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // then: we receive a 400 BAD REQUEST
        assert_eq!(
            response.status(),
            400,
            "The API did not fail with 400 BAD REQUEST when the payload was {error_message}."
        );
    }
}

#[tokio::test]
async fn health_check_works() {
    // given: our app is running
    let TestApp { address, .. } = spawn_app().await;

    // and: an HTTP client
    let client = reqwest::Client::new();

    // when: we make a GET request to /health_check
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to send request.");

    // then: we get an empty success response
    assert_eq!(response.status(), 200);
    assert_eq!(response.content_length(), Some(0));
}

/// Encapsulates data about the test application.
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Starts the application and returns a TestApp, which contains data for the test application.
async fn spawn_app() -> TestApp {
    let mut configuration =
        configuration::get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(configuration.database).await;

    // Binding port 0 causes the OS to bind any available port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener, connection_pool.clone()).expect("Failed to bind address");
    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: connection_pool,
    }
}

/// Creates and migrates the database, returning a connection pool for the database.
async fn configure_database(config: DatabaseSettings) -> PgPool {
    // Create the database.
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database.
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}
