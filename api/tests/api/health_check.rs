use crate::helpers::{random_uuid, spawn_app};

#[tokio::test]
async fn health_check_works() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn secured_health_check_returns_unauthorized_for_missing_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = random_uuid();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(&format!("{}/secured_health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn secured_health_check_returns_unauthorized_for_wrong_token() {
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
        .get(&format!("{}/secured_health_check", &app.address))
        .header("X-Auth-Token", wrong_token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn secured_health_check_works_with_valid_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let project = Project {
        id: 1,
        name: "project_name".to_string(),
        openstack_id: "os_domain_id".to_string(),
        user_class: 1,
    };
    let user = User {
        id: 1,
        name: "user_name".to_string(),
        openstack_id: "os_project_id".to_string(),
        project: 1,
        project_name: "project_name".to_string(),
        is_staff: false,
        is_active: true,
        role: 1,
    };

    let mut transaction = app
        .db_pool
        .begin()
        .await
        .expect("Failed to begin transaction.");
    insert_project_into_db(&mut transaction, &project)
        .await
        .expect("Failed to insert project into database.");
    insert_user_into_db(&mut transaction, &user)
        .await
        .expect("Failed to insert user into database.");
    transaction
        .commit()
        .await
        .expect("Failed to commit transaction.");

    let token = Uuid::new_v4().to_string();
    app.mock_keystone_auth(&token, "os_project_id", "os_project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(&format!("{}/secured_health_check", &app.address))
        .header("X-Auth-Token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 200);
}
