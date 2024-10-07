use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

// TODO: also test master user access
// Permission matrix:
//                     own user     user from own project      other user
//      admin user     X            X                          X
//      master user    X            X                          -
//      normal user    X            -                          -

#[tokio::test]
async fn e2e_lib_user_can_get_own_user() {
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
        let detailed = client.user.get(user.id).unwrap();

        // assert
        assert_eq!(&detailed, &user);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_cannot_get_other_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    let (user2, _project2, _token2) = server
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
        let get = client.user.get(user2.id);

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
async fn e2e_lib_admin_can_get_own_user() {
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
        let detailed = client.user.get(user.id).unwrap();

        // assert
        assert_eq!(detailed, user);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_admin_can_get_other_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    let (user2, _project2, _token2) = server
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
        let detailed = client.user.get(user2.id).unwrap();

        // assert
        assert_eq!(detailed, user2);
    })
    .await
    .unwrap();
}
