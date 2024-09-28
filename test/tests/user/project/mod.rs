use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_number, random_uuid, spawn_app,
};
use lrzcc_wire::user::ProjectRetrieved;
use std::str::FromStr;
use tokio::task::spawn_blocking;

mod create;
mod delete;
mod get;
mod list;
mod modify;

#[tokio::test]
async fn e2e_lib_project_create_and_get_works() {
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

        // act and assert 1 - create
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

        // act and assert 2 - get
        let ProjectRetrieved::Detailed(detailed) =
            client.project.get(created.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };
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

#[tokio::test]
async fn e2e_lib_project_create_and_list_works() {
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

        // act and assert 1 - create
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

        // act and assert 2 - list
        let projects = client.project.list().all().send().unwrap();
        assert_eq!(projects.len(), 2);
        let project2 = &projects[0];
        assert_eq!(project.id, project2.id);
        assert_eq!(project.name, project2.name);
        assert_eq!(project.openstack_id, project2.openstack_id);
        assert_eq!(project.user_class, project2.user_class);
        let project3 = &projects[1];
        assert_eq!(created.id, project3.id);
        assert_eq!(created.name, project3.name);
        assert_eq!(created.openstack_id, project3.openstack_id);
        assert_eq!(created.user_class, project3.user_class);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_project_modify_and_get_works() {
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

        // act and assert 1 - modify
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let user_class = random_number(1..6);
        let modified = client
            .project
            .modify(project.id)
            .name(name.clone())
            .openstack_id(openstack_id.clone())
            .user_class(user_class)
            .send()
            .unwrap();
        assert_eq!(name, modified.name);
        assert_eq!(openstack_id, modified.openstack_id);
        assert_eq!(user_class, modified.user_class);

        // act and assert 2 - get
        let ProjectRetrieved::Detailed(detailed) =
            client.project.get(modified.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };
        assert_eq!(detailed.id, modified.id);
        assert_eq!(detailed.name, modified.name);
        assert_eq!(detailed.openstack_id, modified.openstack_id);
        assert_eq!(detailed.user_class, modified.user_class);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_project_create_get_delete_get_works() {
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

        // act and assert 1 - create
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

        // act and assert 2 - get
        let ProjectRetrieved::Detailed(detailed) =
            client.project.get(created.id).unwrap()
        else {
            panic!("Expected ProjectDetailed")
        };
        assert_eq!(detailed.id, created.id);
        assert_eq!(detailed.name, created.name);
        assert_eq!(detailed.openstack_id, created.openstack_id);
        assert_eq!(detailed.user_class, created.user_class);
        assert_eq!(detailed.users.len(), 0);
        assert_eq!(detailed.flavor_groups.len(), 0);

        // act and assert 3 - delete
        client.project.delete(created.id).unwrap();

        // act and assert 4 - get
        let get = client.project.get(created.id);
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            format!("Project with given ID not found")
        );
    })
    .await
    .unwrap();
}
