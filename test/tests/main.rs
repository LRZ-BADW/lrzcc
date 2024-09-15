use lrzcc::{Api, Token};
use lrzcc_test::spawn_app;
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_hello_user_works() {
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
        let hello = client.hello.user().unwrap();

        // assert
        assert_eq!(
            hello.message,
            format!(
                "Hello, {} from project {} with user class {}",
                user.name, project.name, project.user_class
            )
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_hello_admin_works() {
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
        let hello = client.hello.admin().unwrap();

        // assert
        assert_eq!(
            hello.message,
            format!(
                "Hello, admin {} from project {} with user class {}",
                user.name, project.name, project.user_class
            )
        );
    })
    .await
    .unwrap();
}
