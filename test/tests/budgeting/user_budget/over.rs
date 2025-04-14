use std::str::FromStr;

use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_admin_can_get_user_budget_over_for_all() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client.user_budget.over().all().send();
        assert!(request.is_ok());
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_user_budget_over_for_all() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client.user_budget.over().all().send();
        assert!(request.is_err());
        assert_eq!(
            request.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_get_own_project_budget_over_by_project() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client
            .user_budget
            .over()
            .project(test_project.project.id)
            .send();
        assert!(request.is_ok());
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_project_budget_over_by_project() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();

    let test_project_2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client
            .user_budget
            .over()
            .project(test_project_2.project.id)
            .send();
        assert!(request.is_err());
        assert_eq!(
            request.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_project_budget_over_by_user() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();

    let test_project_2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user = test_project_2.normals[0].user.clone();

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client.user_budget.over().user(normal_user.id).send();
        assert!(request.is_err());
        assert_eq!(
            request.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_can_get_own_user_budget_over_by_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let user_budget = server
        .setup_test_user_budget(&normal_user)
        .await
        .expect("Failed to setup test user budget");

    server
        .mock_keystone_auth(
            &token,
            &normal_user.openstack_id,
            &normal_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client.user_budget.over().budget(user_budget.id).send();
        assert!(request.is_ok());
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_can_get_own_user_budget_over_by_user() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();

    server
        .mock_keystone_auth(
            &token,
            &normal_user.openstack_id,
            &normal_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        let request = client.user_budget.over().user(normal_user.id).send();
        assert!(request.is_ok());
    })
    .await
    .unwrap();
}
