use crate::helpers::spawn_app;
use lrzcc_wire::user::{Project, User};
use uuid::Uuid;

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
    let token = Uuid::new_v4().to_string();
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
    let token = Uuid::new_v4().to_string();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let wrong_token = Uuid::new_v4().to_string();
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

    sqlx::query!(
        r#"
        INSERT INTO user_project (
            id,
            name,
            openstack_id,
            user_class
        )
        VALUES (?, ?, ?, ?)
        "#,
        project.id,
        project.name,
        project.openstack_id,
        project.user_class,
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert project into database.");

    sqlx::query!(
        r#"
        INSERT INTO user_user (
            id,
            password,
            name,
            openstack_id,
            project_id,
            role,
            is_staff,
            is_active
        )
        VALUES (?, "",?, ?, ?, ?, ?, ?)
        "#,
        user.id,
        user.name,
        user.openstack_id,
        user.project,
        user.role,
        user.is_staff,
        user.is_active,
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to insert user into database.");

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
