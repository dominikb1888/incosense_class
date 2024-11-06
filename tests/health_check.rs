#[cfg(test)]
mod tests {

    use std::net::TcpListener;

    #[tokio::test]
    async fn health_check_succeeds() {
        let address = spawn_app().await;
        // We need to bring in `reqwest`
        // to perform HTTP requests against our application.
        let client = reqwest::Client::new();
        // Act
        let response = client
            .get(&format!("{}/health_check", &address))
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    // Launch our application in the background ~somehow~
    async fn spawn_app() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        // We retrieve the port assigned to us by the OS
        let port = listener.local_addr().unwrap().port();
        let server = incosense_class::run(listener).expect("Failed to bind address");
        let _ = tokio::spawn(server);

        // We return the application address to the caller
        format!("http://127.0.0.1:{}", port)
    }

    #[tokio::test]
    async fn subsrcibe_returns_200_for_valid_form_data() {
        let app_address = spawn_app().await;
        let client = reqwest::Client::new();

        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(200, response.status().as_u16());
    }

    #[tokio::test]
    async fn subscribe_returns_a_400_when_data_is_missing() {
        // Arrange
        let app_address = spawn_app().await;
        let client = reqwest::Client::new();
        let test_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            // Act
            let response = client
                .post(&format!("{}/subscriptions", &app_address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message
            );
        }
    }
}
