use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_number, random_uuid, spawn_app,
};
use std::str::FromStr;
use tokio::task::spawn_blocking;

// TODO: also test master user access

#[tokio::test]
async fn e2e_lib_user_list_returns_own_user() {
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
        let users = client.user.list().send().unwrap();

        // assert
        assert_eq!(users.len(), 1);
        let user2 = &users[0];
        assert_eq!(user.id, user2.id);
        assert_eq!(user.name, user2.name);
        assert_eq!(user.openstack_id, user2.openstack_id);
        assert_eq!(user.role, user2.role);
        assert_eq!(user.is_staff, user2.is_staff);
        assert_eq!(user.is_active, user2.is_active);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_list_all_denies_access_to_normal_user() {
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
        let list = client.user.list().all().send();

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
async fn e2e_lib_user_list_by_project_denies_access_to_normal_user() {
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
        let list = client.user.list().project(project.id).send();

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
async fn e2e_lib_user_list_all_works() {
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

        // act part 1 - create projects and users
        let mut projects = Vec::new();
        let mut users = Vec::new();
        projects.push(project);
        users.push(user);
        for _ in 0..5 {
            let name = random_alphanumeric_string(10);
            let openstack_id = random_uuid();
            let user_class = random_number(0..6);
            let project = client
                .project
                .create(name.clone(), openstack_id.clone())
                .user_class(user_class)
                .send()
                .unwrap();
            projects.push(project.clone());

            let name = random_alphanumeric_string(10);
            let openstack_id = random_uuid();
            let user = client
                .user
                .create(name.clone(), openstack_id.clone(), project.id)
                .send()
                .unwrap();
            users.push(user);
        }

        // act part 2 - list all users
        let users2 = client.user.list().all().send().unwrap();

        // assert
        assert_eq!(users.len(), users2.len());
        for (user, user2) in users.into_iter().zip(users2.into_iter()) {
            assert_eq!(user.id, user2.id);
            assert_eq!(user.name, user2.name);
            assert_eq!(user.openstack_id, user2.openstack_id);
            assert_eq!(user.role, user2.role);
            assert_eq!(user.is_staff, user2.is_staff);
            assert_eq!(user.is_active, user2.is_active);
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_list_by_project_works() {
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

        // act part 1 - create projects and users
        let mut projects = Vec::new();
        let mut users = Vec::new();
        projects.push(project);
        users.push(user);
        for i in 0..3 {
            let name = random_alphanumeric_string(10);
            let openstack_id = random_uuid();
            let user_class = random_number(0..6);
            let project = client
                .project
                .create(name.clone(), openstack_id.clone())
                .user_class(user_class)
                .send()
                .unwrap();
            if i == 2 {
                projects.push(project.clone());
            }

            let name = random_alphanumeric_string(10);
            let openstack_id = random_uuid();
            let user = client
                .user
                .create(name.clone(), openstack_id.clone(), project.id)
                .send()
                .unwrap();
            if i == 2 {
                users.push(user);
            }

            let name = random_alphanumeric_string(10);
            let openstack_id = random_uuid();
            let user = client
                .user
                .create(name.clone(), openstack_id.clone(), project.id)
                .send()
                .unwrap();
            if i == 2 {
                users.push(user);
            }
        }

        // act part 2 - list users by project
        let project_id = projects[0].id;
        let users2 = client.user.list().project(project_id).send().unwrap();

        // assert
        assert_eq!(users.len(), users2.len());
        for (user, user2) in users.into_iter().zip(users2.into_iter()) {
            assert_eq!(user.id, user2.id);
            assert_eq!(user.name, user2.name);
            assert_eq!(user.openstack_id, user2.openstack_id);
            assert_eq!(user.role, user2.role);
            assert_eq!(user.is_staff, user2.is_staff);
            assert_eq!(user.is_active, user2.is_active);
        }
    })
    .await
    .unwrap();
}
