use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_user_budget_delete_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&user)
        .await
        .expect("Failed to setup test user budget");

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        // act
        let delete = client.user_budget.delete(user_budget.id);

        // assert
        assert!(delete.is_err());
        assert_eq!(
            delete.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_budget_delete_denies_access_to_master_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&user)
        .await
        .expect("Failed to setup test user budget");

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        // act
        let delete = client.user_budget.delete(user_budget.id);

        // assert
        assert!(delete.is_err());
        assert_eq!(
            delete.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_budget_delete_works() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&user)
        .await
        .expect("Failed to setup test user budget");

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        // act and assert 1 - delete
        client.user_budget.delete(user_budget.id).unwrap();

        // act and assert 2 - get
        let get = client.user_budget.get(user_budget.id);
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
    })
    .await
    .unwrap();
}