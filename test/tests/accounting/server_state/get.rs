use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;

use super::assert_equal_server_states;

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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let retrieved = client.server_state.get(server_state.id).await.unwrap();

    // assert
    assert_equal_server_states(&retrieved, &server_state);
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
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state_2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 1");
    let server_state_3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 2");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    for server_state in [&server_state_2, &server_state_3] {
        // act
        let get = client.server_state.get(server_state.id).await;

        // assert
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
    }
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let retrieved1 = client.server_state.get(server_state_1.id).await.unwrap();
    let retrieved2 = client.server_state.get(server_state_2.id).await.unwrap();

    // assert
    assert_equal_server_states(&retrieved1, &server_state_1);
    assert_equal_server_states(&retrieved2, &server_state_2);
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let get = client.server_state.get(server_state.id).await;

    // assert
    assert!(get.is_err());
    assert_eq!(
        get.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
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
    let server_state_3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 3");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let retrieved1 = client.server_state.get(server_state_1.id).await.unwrap();
    let retrieved2 = client.server_state.get(server_state_2.id).await.unwrap();
    let retrieved3 = client.server_state.get(server_state_3.id).await.unwrap();

    // assert
    assert_equal_server_states(&retrieved1, &server_state_1);
    assert_equal_server_states(&retrieved2, &server_state_2);
    assert_equal_server_states(&retrieved3, &server_state_3);
}
