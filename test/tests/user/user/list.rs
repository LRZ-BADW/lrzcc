use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

// Permission matrix:
//                     no filter    project filter     all filter
//      admin user     X            X                  X
//      master user    X            X                  -
//      normal user    X            -                  -

#[tokio::test]
async fn e2e_lib_normal_user_can_list_own_user() {
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
        let users = client.user.list().send().unwrap();

        // assert
        assert_eq!(users.len(), 1);
        assert_eq!(users[0], user);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_use_user_list_filters() {
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
        let list1 = client.user.list().all().send();
        let list2 = client.user.list().project(project.id).send();

        // assert
        assert!(list1.is_err());
        assert!(list2.is_err());
        assert_eq!(
            list1.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
        assert_eq!(
            list2.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_master_user_can_list_own_projects_users() {
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
    let _test_project2 = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
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
        let users1 = client.user.list().send().unwrap();
        let users2 = client.user.list().project(project.id).send().unwrap();

        // assert
        assert_eq!(users1.len(), 1);
        assert_eq!(users1[0], user);
        assert_eq!(users2.len(), 2);
        assert_eq!(users2[0], user);
        assert_eq!(users2[1], user2);
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
        let users1 = client.user.list().send().unwrap();
        let users2 = client.user.list().project(project.id).send().unwrap();
        let users3 = client.user.list().all().send().unwrap();

        // assert
        assert_eq!(users1.len(), 1);
        assert_eq!(users1[0], user);
        assert_eq!(users2.len(), 2);
        assert_eq!(users2[0], user);
        assert_eq!(users2[1], user2);
        assert_eq!(users3.len(), 4);
        assert_eq!(users3[0], user);
        assert_eq!(users3[1], user2);
        assert_eq!(users3[2], user3);
        assert_eq!(users3[3], user4);
    })
    .await
    .unwrap();
}
