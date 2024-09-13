use lrzcc_api::configuration::{get_configuration, DatabaseSettings};
use lrzcc_api::startup::{get_connection_pool, Application};
use lrzcc_api::telemetry::{get_subscriber, init_subscriber};
use lrzcc_wire::user::{Project, User};
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_json::json;
use sqlx::{
    Connection, Executor, MySql, MySqlConnection, MySqlPool, Transaction,
};
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
    let _ = tokio::spawn(application.run_until_stopped());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address: format!("http://127.0.0.1:{}", application_port),
        _port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        _api_client: client,
        keystone_server,
        keystone_token,
    };
    test_app
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
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_project (
            id,
            name,
            openstack_id,
            user_class
            )
            VALUES (?, ?, ?, ?)
        "#,
        project.id,
        project.name,
        project.openstack_id,
        project.user_class,
    );
    transaction.execute(query).await.map(|_| ())
}

pub async fn insert_user_into_db(
    transaction: &mut Transaction<'static, MySql>,
    user: &User,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_user (
            id,
            password,
            name,
            openstack_id,
            project_id,
            role,
            is_staff,
            is_active
            )
            VALUES (?, "", ?, ?, ?, ?, ?, ?)
        "#,
        user.id,
        user.name,
        user.openstack_id,
        user.project,
        user.role,
        user.is_staff,
        user.is_active,
    );
    transaction.execute(query).await.map(|_| ())
}

fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn random_alphanumeric_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
