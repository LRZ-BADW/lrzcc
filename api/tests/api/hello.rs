use crate::helpers::random_uuid;
use lrzcc_wire::hello::Hello;

use crate::helpers::spawn_app;

#[tokio::test]
async fn hello_returns_unauthorized_for_missing_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = random_uuid();
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
    let token = random_uuid();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let wrong_token = random_uuid();
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
    let token = random_uuid();
    let project_name = "project_name";
    app.mock_keystone_auth(&token, "project_id", project_name)
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
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );
    let hello =
        serde_json::from_str::<Hello>(&response.text().await.unwrap()).unwrap();
    assert_eq!(hello.message, format!("Hello, user {}!", project_name));
}

#[tokio::test]
async fn database_insert_works() {
    // arrange
    let app = spawn_app().await;

    // act and assert
    sqlx::query!(
        "INSERT INTO user_project (name, openstack_id, user_class) VALUES (?, ?, ?)",
        "test",
        "some-uuid",
        4,
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert user.");
}
