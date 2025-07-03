use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;
use avina_wire::user::ProjectRetrieved;

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
        client.project.get(project.id).await.unwrap()
    else {
        panic!("Expected ProjectDetailed")
    };

    // assert
    assert_eq!(project, project_detailed);
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let get = client.project.get(project2.id).await;

    // assert
    assert!(get.is_err());
    // TODO: can be also check the HTTP status code?
    assert_eq!(get.unwrap_err().to_string(), format!("Resource not found"));
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
        client.project.get(project.id).await.unwrap()
    else {
        panic!("Expected ProjectDetailed")
    };

    // assert
    assert_eq!(project_detailed, project);
    assert_eq!(project_detailed.users.len(), 1);
    assert_eq!(project_detailed.users[0], user);
    // TODO: this needs more rigorous testing
    assert_eq!(project_detailed.flavor_groups.len(), 0);
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
        client.project.get(project2.id).await.unwrap()
    else {
        panic!("Expected ProjectDetailed")
    };

    // assert
    assert_eq!(project2, project_detailed);
    assert_eq!(project_detailed.users[0], user2);
    // TODO: this needs more rigorous testing
    assert_eq!(project_detailed.flavor_groups.len(), 0);
}
