use lrzcc::{Api, Token};
use lrzcc_test::{
    random_alphanumeric_string, random_bool, random_number, random_uuid,
    spawn_app,
};
use std::str::FromStr;
use tokio::task::spawn_blocking;

#[tokio::test]
// TODO: also test master user access
async fn e2e_lib_user_modify_denies_access_to_normal_user() {
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
        let modify = client.user.modify(user.id).send();

        // assert
        assert!(modify.is_err());
        assert_eq!(
            modify.unwrap_err().to_string(),
            format!("Admin privileges required")
        );
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn e2e_lib_user_modify_and_get_works() {
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
        let role = random_number(0..3);
        let is_staff = random_bool();
        let is_active = random_bool();
        let modified = client
            .user
            .modify(project.id)
            .name(name.clone())
            .openstack_id(openstack_id.clone())
            .role(role)
            .is_staff(is_staff)
            .is_active(is_active)
            .send()
            .unwrap();
        assert_eq!(name, modified.name);
        assert_eq!(openstack_id, modified.openstack_id);
        assert_eq!(role, modified.role);
        assert_eq!(is_staff, modified.is_staff);
        assert_eq!(is_active, modified.is_active);

        // act and assert 2 - get
        let detailed = client.user.get(modified.id).unwrap();
        assert_eq!(detailed.id, modified.id);
        assert_eq!(detailed.name, modified.name);
        assert_eq!(detailed.openstack_id, modified.openstack_id);
        assert_eq!(detailed.role, modified.role);
        assert_eq!(detailed.is_staff, modified.is_staff);
        assert_eq!(detailed.is_active, modified.is_active);
    })
    .await
    .unwrap();
}
