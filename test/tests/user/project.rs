use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_number, random_uuid, spawn_app,
};
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
async fn e2e_lib_project_create_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
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
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let create = client.project.create(name, openstack_id).send();

        // assert
        assert!(create.is_err());
        assert_eq!(
            create.unwrap_err().to_string(),
            format!("Requesting user is not an admin")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_project_get_for_own_project_of_admin() {
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
        let project_detailed = client.project.get(project.id).unwrap();

        // assert
        assert_eq!(project.id, project_detailed.id);
        assert_eq!(project.name, project_detailed.name);
        assert_eq!(project.openstack_id, project_detailed.openstack_id);
        assert_eq!(project.user_class, project_detailed.user_class);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_project_create_and_get_works() {
    // Arrange
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
        // Arrange
        let client = Api::new(
            format!("{}/api", &server.address),
            Token::from_str(&token).unwrap(),
            None,
            None,
        )
        .unwrap();

        // Act and Assert 1
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let user_class = random_number(1..6);
        let created = client
            .project
            .create(name.clone(), openstack_id.clone())
            .user_class(user_class)
            .send()
            .unwrap();
        assert_eq!(name, created.name);
        assert_eq!(openstack_id, created.openstack_id);
        assert_eq!(user_class, created.user_class);

        // Act and Assert 2
        let detailed = client.project.get(created.id).unwrap();
        assert_eq!(detailed.id, created.id);
        assert_eq!(detailed.name, created.name);
        assert_eq!(detailed.openstack_id, created.openstack_id);
        assert_eq!(detailed.user_class, created.user_class);
        assert_eq!(detailed.users.len(), 0);
        assert_eq!(detailed.flavor_groups.len(), 0);
    })
    .await
    .unwrap();
}
