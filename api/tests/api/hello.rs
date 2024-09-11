use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn hello_returns_unauthorized_for_missing_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = Uuid::new_v4().to_string();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(&format!("{}/hello", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn hello_returns_unauthorized_for_wrong_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = Uuid::new_v4().to_string();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let wrong_token = Uuid::new_v4().to_string();
    let response = client
        .get(&format!("{}/hello", &app.address))
        .header("X-Auth-Token", wrong_token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn hello_works_with_valid_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = Uuid::new_v4().to_string();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(&format!("{}/hello", &app.address))
        .header("X-Auth-Token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 200);
}
