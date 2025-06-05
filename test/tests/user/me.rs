use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_user_me_works() {
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
        let me = client.user.me().unwrap();

        // assert
        assert_eq!(me.id, user.id);
        assert_eq!(me.name, user.name);
        assert_eq!(me.openstack_id, user.openstack_id);
        assert_eq!(me.role, user.role);
        assert_eq!(me.is_staff, user.is_staff);
        assert_eq!(me.is_active, user.is_active);
        assert_eq!(me.project.id, project.id);
        assert_eq!(me.project.name, project.name);
        assert_eq!(me.project_name, project.name);
    })
    .await
    .unwrap();
}
