use std::net::TcpListener;

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

// Starts the application and returns the address at which it is available.
fn spawn_app() -> String {
    // Binding port 0 causes the OS to bind any available port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
