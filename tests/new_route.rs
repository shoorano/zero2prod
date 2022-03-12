use std::net::TcpListener;

/// Launch our application in the background
/// and returns its address (i.e. http://localhost::XXXX)
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind to a random port");

    // retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn new_route_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    // to perform Http requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/new_route", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
