use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_bool, random_number, random_uuid,
    spawn_app,
};
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
// TODO: also test master user access
async fn e2e_lib_user_create_denies_access_to_normal_user() {
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
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let create = client.user.create(name, openstack_id, project.id).send();

        // assert
        assert!(create.is_err());
        assert_eq!(
            create.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_create_works() {
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
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let role = random_number(0..3);
        let is_staff = random_bool();
        let is_active = random_bool();
        let mut request =
            client
                .user
                .create(name.clone(), openstack_id.clone(), project.id);
        request.role(role);
        if is_staff {
            request.staff();
        }
        if !is_active {
            request.inactive();
        }
        let created = request.send().unwrap();

        // assert
        assert_eq!(name, created.name);
        assert_eq!(openstack_id, created.openstack_id);
        assert_eq!(project.id, created.project);
        assert_eq!(project.name, created.project_name);
        assert_eq!(role, created.role);
        assert_eq!(is_staff, created.is_staff);
        assert_eq!(is_active, created.is_active);
    })
    .await
    .unwrap();
}

// TODO: how can we test internal server error responses?
#[tokio::test]
async fn e2e_lib_user_create_twice_returns_bad_request() {
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
        let role = random_number(0..3);
        let is_staff = random_bool();
        let is_active = random_bool();
        let mut request =
            client
                .user
                .create(name.clone(), openstack_id.clone(), project.id);
        request.role(role);
        if is_staff {
            request.staff();
        }
        if !is_active {
            request.inactive();
        }
        let created = request.send().unwrap();
        assert_eq!(name, created.name);
        assert_eq!(openstack_id, created.openstack_id);
        assert_eq!(project.id, created.project);
        assert_eq!(project.name, created.project_name);
        assert_eq!(role, created.role);
        assert_eq!(is_staff, created.is_staff);
        assert_eq!(is_active, created.is_active);

        // act and assert 2 - create
        let create = client
            .user
            .create(name.clone(), openstack_id.clone(), project.id)
            .send();
        match create {
            Err(lrzcc::error::ApiError::ResponseError(message)) => {
                assert_eq!(
                    message,
                    "Failed to insert new user, a conflicting entry exists"
                        .to_string()
                );
            }
            _ => panic!("Expected ApiError::ResponseError"),
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_create_and_get_works() {
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
        let role = random_number(0..3);
        let is_staff = random_bool();
        let is_active = random_bool();
        let mut request =
            client
                .user
                .create(name.clone(), openstack_id.clone(), project.id);
        request.role(role);
        if is_staff {
            request.staff();
        }
        if !is_active {
            request.inactive();
        }
        let created = request.send().unwrap();
        assert_eq!(name, created.name);
        assert_eq!(openstack_id, created.openstack_id);
        assert_eq!(project.id, created.project);
        assert_eq!(project.name, created.project_name);
        assert_eq!(role, created.role);
        assert_eq!(is_staff, created.is_staff);
        assert_eq!(is_active, created.is_active);

        // act and assert 2 - get
        let detailed = client.user.get(created.id).unwrap();
        assert_eq!(detailed.id, created.id);
        assert_eq!(detailed.name, created.name);
        assert_eq!(detailed.openstack_id, created.openstack_id);
        assert_eq!(detailed.project.id, created.project);
        assert_eq!(detailed.project.name, created.project_name);
        assert_eq!(detailed.project_name, created.project_name);
        assert_eq!(detailed.role, created.role);
        assert_eq!(detailed.is_staff, created.is_staff);
        assert_eq!(detailed.is_active, created.is_active);
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_create_and_list_works() {
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
        let role = random_number(0..3);
        let is_staff = random_bool();
        let is_active = random_bool();
        let mut request =
            client
                .user
                .create(name.clone(), openstack_id.clone(), project.id);
        request.role(role);
        if is_staff {
            request.staff();
        }
        if !is_active {
            request.inactive();
        }
        let created = request.send().unwrap();
        assert_eq!(name, created.name);
        assert_eq!(openstack_id, created.openstack_id);
        assert_eq!(project.id, created.project);
        assert_eq!(project.name, created.project_name);
        assert_eq!(role, created.role);
        assert_eq!(is_staff, created.is_staff);
        assert_eq!(is_active, created.is_active);

        // act and assert 2 - list
        let users = client.user.list().all().send().unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(user, users[0]);
        assert_eq!(created, users[1]);
    })
    .await
    .unwrap();
}
