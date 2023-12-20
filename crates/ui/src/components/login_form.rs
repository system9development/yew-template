async fn post_login(
    username: UseStateHandle<String>,
    password: UseStateHandle<String>,
) -> Result<String> {
    let client = reqwest::Client::new();
    let username = username.to_string();
    let password = password.to_string();

    let login_post_data = LoginPostData { username, password };

    let response = client
        .post(format!("{}/login", BACKEND_URL))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string_pretty(&login_post_data)?)
        .send()
        .await?;
    Ok(response.text().await?)
}
