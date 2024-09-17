use crate::authentication::{
    extract_user_and_project, require_admin_user, require_valid_token,
};
use crate::configuration::{DatabaseSettings, Settings};
use crate::openstack::OpenStack;
use crate::routes::{health_check, hello_admin, hello_user};
use actix_web::{
    dev::Server, middleware::from_fn, web, web::Data, App, HttpServer,
};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let openstack = OpenStack::new(configuration.openstack).await?;

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            openstack,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    base_url: String,
    openstack: OpenStack,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let openstack = Data::new(openstack);
    // TODO add default service for proper 404
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(openstack.clone())
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .wrap(from_fn(extract_user_and_project))
                    .wrap(from_fn(require_valid_token))
                    .route("/secured_health_check", web::get().to(health_check))
                    .route("/hello", web::get().to(hello_user))
                    .service(
                        web::scope("")
                            .wrap(from_fn(require_admin_user))
                            .route("/hello/admin", web::get().to(hello_admin)),
                    ),
            )
        // )
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> MySqlPool {
    MySqlPoolOptions::new().connect_lazy_with(configuration.with_db())
}
