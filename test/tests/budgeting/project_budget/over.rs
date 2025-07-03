use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;

#[tokio::test]
async fn e2e_lib_admin_can_get_project_budget_over_for_all() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client.project_budget.over().all().send().await;

    assert!(request.is_ok());
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_project_budget_over_for_all() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client.project_budget.over().all().send().await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Admin privileges required".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_user_can_get_own_project_budget_over_by_project() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project;

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .over()
        .project(project.id)
        .send()
        .await;

    assert!(request.is_ok());
}

#[tokio::test]
async fn e2e_lib_user_can_get_own_project_budget_over_by_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project;

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;
    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .over()
        .budget(project_budget.id)
        .send()
        .await;

    assert!(request.is_ok());
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_project_budget_over_by_project() {
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();

    let test_project_2 = server
        .setup_test_project(0, 0, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .over()
        .project(project_2.id)
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_project_budget_over_by_budget() {
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();

    let test_project_2 = server
        .setup_test_project(0, 0, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    let project_budget_2 = server
        .setup_test_project_budget(&project_2)
        .await
        .expect("Failed to setup test user budget");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .over()
        .budget(project_budget_2.id)
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}
