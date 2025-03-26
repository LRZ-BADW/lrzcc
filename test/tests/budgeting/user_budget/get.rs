use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_admin_can_get_user_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let normal_user = test_project.normals[0].user.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&normal_user)
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

        let get = client.user_budget.get(user_budget.id);

        assert_eq!(user_budget.id, get.unwrap().id);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_can_get_own_user_budget() {
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
    let user_budget = server
        .setup_test_user_budget(&normal_user)
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

        let get = client.user_budget.get(user_budget.id);
        assert_eq!(get.unwrap().id, user_budget.id);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_cannot_get_other_user_budget() {
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 0, 2)
        .await
        .expect("Failed to setup test project");
    let normal_user_1 = test_project_1.normals[0].user.clone();
    let token = test_project_1.normals[0].token.clone();
    let normal_user_2 = test_project_1.normals[1].user.clone();

    server
        .mock_keystone_auth(
            &token,
            &normal_user_1.openstack_id,
            &normal_user_1.name,
        )
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&normal_user_2)
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

        let get = client.user_budget.get(user_budget.id);
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            "Admin privileges, master user of project or respective user required".to_string()
        );

    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_get_own_project_user_budgets() {
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
    let user_budget = server
        .setup_test_user_budget(&master_user)
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

        let get = client.user_budget.get(user_budget.id);
        assert_eq!(get.unwrap().id, user_budget.id);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_get_other_project_user_budgets() {
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let normal_user = test_project_1.normals[0].user.clone();
    let token = test_project_1.masters[0].token.clone();

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;
    let user_budget = server
        .setup_test_user_budget(&normal_user)
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

        let get = client.user_budget.get(user_budget.id);
        assert_eq!(get.unwrap().id, user_budget.id);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_project_user_budgets() {
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();

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
    let user_budget = server
        .setup_test_user_budget(&normal_user)
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

        let get = client.user_budget.get(user_budget.id);
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            "Admin privileges, master user of project or respective user required".to_string()
        );

    })
    .await
    .unwrap();
}
