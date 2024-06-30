use std::net::TcpListener;
use zero2prod::startup;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // given: our app is running
    let address = spawn_app();

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
}

#[tokio::test]
async fn subscribe_returns_a_400_when_a_required_parameter_is_missing() {
    // given: our app is running
    let address = spawn_app();

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
    let address = spawn_app();

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

/// Starts the application and returns the address at which it is available.
fn spawn_app() -> String {
    // Binding port 0 causes the OS to bind any available port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a port");
    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
