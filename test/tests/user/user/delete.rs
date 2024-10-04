use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_bool, random_number, random_uuid,
    spawn_app,
};
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
// TODO: also test master user access
async fn e2e_lib_user_delete_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and user.");
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
        let delete = client.user.delete(user.id);

        // assert
        assert!(delete.is_err());
        assert_eq!(
            delete.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_create_get_delete_get_works() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and user.");
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
        assert_eq!(detailed.name, created.name);
        assert_eq!(detailed.openstack_id, created.openstack_id);
        assert_eq!(detailed.project.id, created.project);
        assert_eq!(detailed.project.name, created.project_name);
        assert_eq!(detailed.project_name, created.project_name);
        assert_eq!(detailed.role, created.role);
        assert_eq!(detailed.is_staff, created.is_staff);
        assert_eq!(detailed.is_active, created.is_active);

        // act and assert 3 - delete
        client.user.delete(created.id).unwrap();

        // act and assert 4 - get
        let get = client.user.get(created.id);
        assert!(get.is_err());
        assert_eq!(
            get.unwrap_err().to_string(),
            format!("User with given ID or linked project not found")
        );
    })
    .await
    .unwrap();
}
