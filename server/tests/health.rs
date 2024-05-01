mod helpers;

#[tokio::test]
async fn health_check_works() {
    // Start server
    let app = helpers::spawn_app().await;
    println!("Base URL: {}", app.base_url);

    // Send request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", &app.base_url))
        .send()
        .await
        .expect("Failed to execute request");

    // Validate results
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
