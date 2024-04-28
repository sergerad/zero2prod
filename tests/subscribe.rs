mod helpers;

#[tokio::test]
async fn subscribe_retuns_200_valid_form_data() {
    // Start server
    let base_url = helpers::spawn_app().await;
    println!("Base URL: {}", base_url);

    // Send request
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{base_url}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Validate results
    assert_eq!(200, response.status().as_u16());
}

async fn subscribe_retuns_400_missing_form_data() {
    // Start server
    let base_url = helpers::spawn_app().await;
    println!("Base URL: {}", base_url);

    // Instantiate client
    let client = reqwest::Client::new();

    // Define test cases
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Send requests
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{base_url}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Validate results
        assert_eq!(
            400,
            response.status().as_u16(),
            "API did not fail with 400 when payload was {error_message}"
        );
    }
}
