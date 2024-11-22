use lrzcc_api::configuration::{get_configuration, DatabaseSettings};
use lrzcc_api::startup::{get_connection_pool, Application};
use lrzcc_api::telemetry::{get_subscriber, init_subscriber};
use lrzcc_wire::resources::Flavor;
use lrzcc_wire::user::{Project, User};
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{
    Connection, Executor, MySql, MySqlConnection, MySqlPool, Transaction,
};
use std::ops::Range;
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub _port: u16,
    pub db_pool: sqlx::MySqlPool,
    pub _api_client: reqwest::Client,
    pub keystone_server: MockServer,
    pub keystone_token: String,
}

pub struct TestUser {
    pub user: User,
    pub token: String,
}

pub struct TestProject {
    pub project: Project,
    pub admins: Vec<TestUser>,
    pub masters: Vec<TestUser>,
    pub normals: Vec<TestUser>,
}

impl TestApp {
    pub fn mock_keystone_auth(
        &self,
        token: &str,
        os_project_id: &str,
        os_project_name: &str,
    ) -> Mock {
        Mock::given(method("GET"))
            .and(path("/auth/tokens/"))
            .and(header("X-Subject-Token", token))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header("X-Subject-Token", &self.keystone_token)
                    .set_body_json(json!({
                        "token": {
                            "project": {
                                "id": os_project_id,
                                "name": os_project_name,
                            }
                        }
                    })),
            )
    }

    pub async fn setup_test_user_and_project(
        &self,
        admin: bool,
    ) -> Result<(User, Project, String), sqlx::Error> {
        let numbers = if admin { (1, 0, 0) } else { (0, 0, 1) };
        let test_project = self
            .setup_test_project(numbers.0, numbers.1, numbers.2)
            .await?;
        let test_user = if admin {
            &test_project.admins[0]
        } else {
            &test_project.normals[0]
        };
        Ok((
            test_user.user.clone(),
            test_project.project,
            test_user.token.clone(),
        ))
    }

    pub async fn setup_test_user(
        &self,
        transaction: &mut Transaction<'static, MySql>,
        project: &Project,
        is_staff: bool,
        role: u32,
    ) -> Result<TestUser, sqlx::Error> {
        let mut user = User {
            id: 1,
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            project: project.id,
            project_name: project.name.clone(),
            is_staff,
            is_active: true,
            role,
        };
        user.id = insert_user_into_db(transaction, &user).await? as u32;
        let token = Uuid::new_v4().to_string();
        Ok(TestUser { user, token })
    }

    pub async fn setup_test_project(
        &self,
        admin_number: usize,
        master_number: usize,
        normal_number: usize,
    ) -> Result<TestProject, sqlx::Error> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");

        let mut project = Project {
            id: 1,
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            user_class: random_number(1..6),
        };
        project.id =
            insert_project_into_db(&mut transaction, &project).await? as u32;

        let mut test_project = TestProject {
            project,
            admins: Vec::new(),
            masters: Vec::new(),
            normals: Vec::new(),
        };

        for _ in 0..admin_number {
            test_project.admins.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    true,
                    1,
                )
                .await?,
            );
        }

        for _ in 0..master_number {
            test_project.masters.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    false,
                    2,
                )
                .await?,
            );
        }

        for _ in 0..normal_number {
            test_project.normals.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    false,
                    1,
                )
                .await?,
            );
        }

        transaction.commit().await?;

        Ok(test_project)
    }

    pub async fn setup_test_flavor(&self) -> Result<Flavor, sqlx::Error> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let mut flavor = Flavor {
            id: 1,
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            group: None,
            group_name: None,
            weight: 0,
        };
        flavor.id =
            insert_flavor_into_db(&mut transaction, &flavor).await? as u32;
        transaction.commit().await?;
        Ok(flavor)
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let keystone_server = MockServer::start().await;
    let keystone_token = Uuid::new_v4().to_string();

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().simple().to_string();
        c.application.port = 0;
        c.openstack.keystone_endpoint = keystone_server.uri();
        c.application.insert_admin = false;
        c
    };

    configure_database(&configuration.database).await;

    Mock::given(method("POST"))
        .and(path("/auth/tokens/"))
        .respond_with(
            ResponseTemplate::new(201)
                .append_header("X-Subject-Token", &keystone_token),
        )
        .mount(&keystone_server)
        .await;
    // TODO check data sent to keystone

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(application.run_until_stopped());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
        _port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        _api_client: client,
        keystone_server,
        keystone_token,
    }
}

async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    // Create database
    let mut connection = MySqlConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to MariaDB.");
    connection
        .execute(
            format!(
                "CREATE DATABASE IF NOT EXISTS `{}`;",
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = MySqlPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to MariaDB.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}

pub async fn insert_project_into_db(
    transaction: &mut Transaction<'static, MySql>,
    project: &Project,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_project (
            name,
            openstack_id,
            user_class
            )
            VALUES (?, ?, ?)
        "#,
        project.name,
        project.openstack_id,
        project.user_class,
    );
    transaction
        .execute(query)
        .await
        .map(|result| result.last_insert_id())
}

pub async fn insert_user_into_db(
    transaction: &mut Transaction<'static, MySql>,
    user: &User,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_user (
            password,
            name,
            openstack_id,
            project_id,
            role,
            is_staff,
            is_active
            )
            VALUES ("", ?, ?, ?, ?, ?, ?)
        "#,
        user.name,
        user.openstack_id,
        user.project,
        user.role,
        user.is_staff,
        user.is_active,
    );
    transaction
        .execute(query)
        .await
        .map(|result| result.last_insert_id())
}

pub async fn insert_flavor_into_db(
    transaction: &mut Transaction<'static, MySql>,
    flavor: &Flavor,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO resources_flavor (
            name,
            openstack_id,
            group_id,
            weight
            )
            VALUES (?, ?, ?, ?)
        "#,
        flavor.name,
        flavor.openstack_id,
        flavor.group,
        flavor.weight,
    );
    transaction
        .execute(query)
        .await
        .map(|result| result.last_insert_id())
}

pub fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_alphanumeric_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_number(range: Range<u32>) -> u32 {
    thread_rng().gen_range(range)
}

pub fn random_bool() -> bool {
    thread_rng().gen_bool(0.5)
}
