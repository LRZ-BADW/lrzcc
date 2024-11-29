use super::assert_contains_server_state;
use lrzcc::{Api, Token};
use lrzcc_test::{random_uuid, spawn_app};
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_normal_user_can_list_own_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 2)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let user2 = test_project.normals[1].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 2");
    let _server_state3 = server
        .setup_test_server_state(&flavor, &user2)
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
        let server_states = client.server_state.list().send().unwrap();

        // assert
        assert_eq!(server_states.len(), 2);
        assert_contains_server_state(&server_states, &server_state1);
        assert_contains_server_state(&server_states, &server_state2);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_use_other_server_state_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
    let token = test_project.normals[0].token.clone();
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
        let list1 = client.server_state.list().all().send();
        let list2 = client.server_state.list().user(user.id).send();
        let list3 = client.server_state.list().project(project.id).send();

        // assert
        assert!(list1.is_err());
        assert!(list2.is_err());
        assert!(list3.is_err());
        assert_eq!(
            list1.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
        assert_eq!(
            list2.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
        assert_eq!(
            list3.unwrap_err().to_string(),
            format!("Admin or master user privileges for respective project required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_use_other_server_state_all_filter() {
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
        let list = client.server_state.list().all().send();

        // assert
        assert!(list.is_err());
        assert_eq!(
            list.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_list_own_projects_and_users_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
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
    let server_state1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 1");
    let _server_state3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 1");

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
        let server_states1 = client.server_state.list().send().unwrap();
        let server_states2 =
            client.server_state.list().user(user2.id).send().unwrap();
        let server_states3 = client
            .server_state
            .list()
            .project(project.id)
            .send()
            .unwrap();

        // assert
        assert_eq!(server_states1.len(), 1);
        assert_contains_server_state(&server_states1, &server_state1);
        assert_eq!(server_states2.len(), 1);
        assert_contains_server_state(&server_states2, &server_state2);
        assert_eq!(server_states3.len(), 2);
        assert_contains_server_state(&server_states3, &server_state1);
        assert_contains_server_state(&server_states3, &server_state2);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_server_state_list_server_filter_stays_within_project_for_master_user(
) {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
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
    let server_id = random_uuid();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, &server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, &server_id)
        .await
        .expect("Failed to setup test server state 2");
    let _server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, &server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

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
        let server_states = client
            .server_state
            .list()
            .server(&server_id)
            .send()
            .unwrap();

        // assert
        assert_eq!(server_states.len(), 2);
        assert_contains_server_state(&server_states, &server_state1);
        assert_contains_server_state(&server_states, &server_state2);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_server_state_list_server_filter_works_across_projects_for_admin_user(
) {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
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
    let server_id = random_uuid();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, &server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, &server_id)
        .await
        .expect("Failed to setup test server state 2");
    let server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, &server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

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
        let server_states = client
            .server_state
            .list()
            .server(&server_id)
            .send()
            .unwrap();

        // assert
        assert_eq!(server_states.len(), 3);
        assert_contains_server_state(&server_states, &server_state1);
        assert_contains_server_state(&server_states, &server_state2);
        assert_contains_server_state(&server_states, &server_state3);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_admin_user_can_use_any_user_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
    let test_project2 = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.masters[0].user.clone();
    let user4 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 2");
    let server_state3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 3");
    let server_state4 = server
        .setup_test_server_state(&flavor, &user4)
        .await
        .expect("Failed to setup test server state 4");
    let server_state5 = server
        .setup_test_server_state(&flavor, &user4)
        .await
        .expect("Failed to setup test server state 5");

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
        let server_states1 = client.server_state.list().send().unwrap();
        let server_states2 = client
            .server_state
            .list()
            .server(&server_state5.instance_id)
            .send()
            .unwrap();
        let server_states3 =
            client.server_state.list().user(user2.id).send().unwrap();
        let server_states4 = client
            .server_state
            .list()
            .project(project.id)
            .send()
            .unwrap();
        let server_states5 = client.server_state.list().all().send().unwrap();

        // assert
        assert_eq!(server_states1.len(), 1);
        assert_contains_server_state(&server_states1, &server_state1);
        assert_eq!(server_states2.len(), 1);
        assert_contains_server_state(&server_states2, &server_state5);
        assert_eq!(server_states3.len(), 1);
        assert_contains_server_state(&server_states3, &server_state2);
        assert_eq!(server_states4.len(), 2);
        assert_contains_server_state(&server_states4, &server_state1);
        assert_contains_server_state(&server_states4, &server_state2);
        assert_eq!(server_states5.len(), 5);
        assert_contains_server_state(&server_states5, &server_state1);
        assert_contains_server_state(&server_states5, &server_state2);
        assert_contains_server_state(&server_states5, &server_state3);
        assert_contains_server_state(&server_states5, &server_state4);
        assert_contains_server_state(&server_states5, &server_state5);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_combine_server_state_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project.clone();
    let user2 = test_project.normals[0].user.clone();
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
    let server_id = random_uuid();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, &server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, &server_id)
        .await
        .expect("Failed to setup test server state 2");
    let _server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, &server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

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
        let server_states1 = client
            .server_state
            .list()
            .project(project.id)
            .server(&server_id)
            .send()
            .unwrap();
        let server_states2 = client
            .server_state
            .list()
            .user(user.id)
            .server(&server_id)
            .send()
            .unwrap();

        // assert
        assert_eq!(server_states1.len(), 2);
        assert_contains_server_state(&server_states1, &server_state1);
        assert_contains_server_state(&server_states1, &server_state2);
        assert_eq!(server_states2.len(), 1);
        assert_contains_server_state(&server_states2, &server_state1);
    })
    .await
    .unwrap();
}
