
use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

// Permission matrix:
//                     own state    state from own project     other state
//      admin user     X            X                          X
//      master user    X            X                          -
//      normal user    X            -                          -

#[tokio::test]
async fn e2e_lib_normal_user_can_get_own_server_state() {
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
    let server_state = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state");

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
        let retrieved = client.server_state.get(server_state.id).unwrap();

        // assert
        assert_eq!(&retrieved, &server_state);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_get_other_server_state() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 2)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let user2 = test_project.normals[1].user.clone();
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server.setup_test_flavor().await.expect("Failed to setup test flavor");
    let server_state_2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 1");
    let server_state_3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 2");

    spawn_blocking(move || {
        // arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        for server_state in vec![&server_state_2, &server_state_3] {
            // act
            let get = client.server_state.get(server_state.id);

            // assert
            assert!(get.is_err());
            assert_eq!(
                get.unwrap_err().to_string(),
                format!("Admin or master user privileges for respective project required")
            );
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_get_own_projects_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state_1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state_2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 2");

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
        let retrieved1 = client.server_state.get(server_state_1.id).unwrap();
        let retrieved2 = client.server_state.get(server_state_2.id).unwrap();

        // assert
        assert_eq!(&retrieved1, &server_state_1);
        assert_eq!(&retrieved2, &server_state_2);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_projects_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let user2 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state");

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
        let get = client.server_state.get(server_state.id);

        // assert
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            // TODO: should'nt these cases return a non found error?
            format!("Admin or master user privileges for respective project required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_admin_can_get_all_kinds_of_users() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    let user3 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server.setup_test_flavor().await.expect("Failed to setup test flavor");
    let server_state_1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state_2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 2");
    let server_state_3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 3");

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
        let retrieved1 = client.server_state.get(server_state_1.id).unwrap();
        let retrieved2 = client.server_state.get(server_state_2.id).unwrap();
        let retrieved3 = client.server_state.get(server_state_3.id).unwrap();

        // assert
        assert_eq!(retrieved1, server_state_1);
        assert_eq!(retrieved2, server_state_2);
        assert_eq!(retrieved3, server_state_3);
    })
    .await
    .unwrap();
}
