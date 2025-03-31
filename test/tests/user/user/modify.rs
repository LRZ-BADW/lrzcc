use std::str::FromStr;

use lrzcc::{Api, Token};
use lrzcc_test::{random_uuid, spawn_app};
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_user_modify_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
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

        // act
        let modify = client.user.modify(user.id).send();

        // assert
        assert!(modify.is_err());
        assert_eq!(
            modify.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_modify_denies_access_to_master_user() {
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
        let modify = client.user.modify(user.id).send();

        // assert
        assert!(modify.is_err());
        assert_eq!(
            modify.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_modify_and_get_works() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
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

        // act and assert 1 - modify
        let openstack_id = random_uuid();
        let modified = client
            .user
            .modify(user.id)
            .openstack_id(openstack_id.clone())
            .send()
            .unwrap();
        assert_eq!(user.id, modified.id);
        assert_eq!(user.name, modified.name);
        assert_eq!(openstack_id, modified.openstack_id);
        assert_eq!(user.role, modified.role);
        assert_eq!(project.id, modified.project);
        assert_eq!(project.name, modified.project_name);
        assert_eq!(user.is_staff, modified.is_staff);
        assert_eq!(user.is_active, modified.is_active);
        assert_eq!(user.is_active, modified.is_active);

        // act and assert 2 - get
        let detailed = client.user.get(modified.id).unwrap();
        assert_eq!(detailed, modified);
    })
    .await
    .unwrap();
}
