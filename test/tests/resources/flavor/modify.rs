use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_flavor_modify_denies_access_to_normal_user() {
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
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

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
        let modify = client.flavor.modify(flavor.id).send();

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
async fn e2e_lib_flavor_modify_denies_access_to_master_user() {
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
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

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
        let modify = client.flavor.modify(flavor.id).send();

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
async fn e2e_lib_flavor_modify_and_get_works() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

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
        let name = random_alphanumeric_string(10);
        let modified = client
            .flavor
            .modify(flavor.id)
            .openstack_id(openstack_id.clone())
            .name(name.clone())
            .send()
            .unwrap();
        assert_eq!(openstack_id, modified.openstack_id);
        assert_eq!(name, modified.name);

        // act and assert 2 - get
        let retrieved = client.flavor.get(modified.id).unwrap();
        assert_eq!(modified.id, retrieved.id);
        assert_eq!(modified.openstack_id, retrieved.openstack_id);
        assert_eq!(modified.name, retrieved.name);
        // TODO: compare group
        assert_eq!(modified.group_name, retrieved.group_name);
        assert_eq!(modified.weight, retrieved.weight);
    })
    .await
    .unwrap();
}
