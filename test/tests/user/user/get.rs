use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;

// Permission matrix:
//                     own user     user from own project      other user
//      admin user     X            X                          X
//      master user    X            X                          -
//      normal user    X            -                          -

#[tokio::test]
async fn e2e_lib_normal_user_can_get_own_user() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let detailed = client.user.get(user.id).await.unwrap();

    // assert
    assert_eq!(&detailed, &user);
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_get_other_users() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    for user in [&user2, &user3] {
        // act
        let get = client.user.get(user.id).await;

        // assert
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            "Resource not found".to_string()
        );
    }
}

#[tokio::test]
async fn e2e_lib_master_user_can_get_own_projects_users() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let detailed = client.user.get(user.id).await.unwrap();
    let detailed2 = client.user.get(user2.id).await.unwrap();

    // assert
    assert_eq!(&detailed, &user);
    assert_eq!(&detailed2, &user2);
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_get_other_projects_users() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let get = client.user.get(user2.id).await;

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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let detailed = client.user.get(user.id).await.unwrap();
    let detailed2 = client.user.get(user2.id).await.unwrap();
    let detailed3 = client.user.get(user3.id).await.unwrap();

    // assert
    assert_eq!(detailed, user);
    assert_eq!(detailed2, user2);
    assert_eq!(detailed3, user3);
}
