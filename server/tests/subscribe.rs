mod helpers;

#[tokio::test]
async fn subscribe_retuns_200_valid_form_data() {
    let app = helpers::spawn_app().await;

    // Send request
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.base_url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Validate request result
    assert_eq!(200, response.status().as_u16());

    // Validate saved data
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved data");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_retuns_400_missing_form_data() {
    // Start server
    let app = helpers::spawn_app().await;
    println!("Base URL: {}", app.base_url);

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
            .post(format!("{}/subscriptions", app.base_url))
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
