use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use lrzcc_wire::user::ProjectRetrieved;
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_user_can_get_own_project() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
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
        let ProjectRetrieved::Normal(project_detailed) =
            client.project.get(project.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };

        // assert
        assert_eq!(project, project_detailed);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_cannot_get_other_project() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    let (_user2, project2, _token2) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user2 and project.");
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
        let get = client.project.get(project2.id);

        // assert
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_admin_can_get_own_project() {
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

        // act
        let ProjectRetrieved::Detailed(project_detailed) =
            client.project.get(project.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };

        // assert
        assert_eq!(project_detailed, project);
        assert_eq!(project_detailed.users.len(), 1);
        assert_eq!(project_detailed.users[0], user);
        // TODO: this needs more rigorous testing
        assert_eq!(project_detailed.flavor_groups.len(), 0);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_admin_can_get_other_project() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    let (user2, project2, _token2) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user2 and project.");
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
        let ProjectRetrieved::Detailed(project_detailed) =
            client.project.get(project2.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };

        // assert
        assert_eq!(project2, project_detailed);
        assert_eq!(project_detailed.users[0], user2);
        // TODO: this needs more rigorous testing
        assert_eq!(project_detailed.flavor_groups.len(), 0);
    })
    .await
    .unwrap();
}
