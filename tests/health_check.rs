use std::net::TcpListener;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap().to_string();
    let server = zero2prod::run(listener)
        .await
        .expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://{addr}")
}

#[tokio::test]
async fn health_check_works() {
    let base_url = spawn_app().await;
    println!("Base URL: {}", base_url);
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{base_url}/health"))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(13), response.content_length());
}
