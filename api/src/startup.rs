use crate::authentication::{extract_user_and_project, require_valid_token};
use crate::configuration::{DatabaseSettings, Settings};
use crate::error::not_found;
use crate::error::MinimalApiError;
use crate::openstack::OpenStack;
use crate::routes::user::project::create::{
    insert_project_into_db, NewProject,
};
use crate::routes::user::user::create::{insert_user_into_db, NewUser};
use crate::routes::{accounting_scope, health_check, hello_scope, user_scope};
use actix_web::{
    dev::Server, middleware::from_fn, web, web::Data, App, HttpServer,
};
use anyhow::Context;
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

        if configuration.application.insert_admin {
            Self::insert_admin_user(&connection_pool, &configuration).await?;
        }

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

    async fn insert_admin_user(
        connection_pool: &MySqlPool,
        configuration: &Settings,
    ) -> Result<(), anyhow::Error> {
        let mut transaction = connection_pool
            .begin()
            .await
            .context("Failed to begin transaction")?;
        let project = NewProject {
            name: configuration.openstack.domain.clone(),
            openstack_id: configuration.openstack.domain_id.clone(),
            user_class: 1,
        };
        let project_id =
            match insert_project_into_db(&mut transaction, &project).await {
                Ok(project_id) => project_id,
                Err(MinimalApiError::ValidationError(_)) => {
                    tracing::info!("Admin project already exists, skipping.");
                    return Ok(());
                }
                Err(MinimalApiError::UnexpectedError(e)) => {
                    return Err(e);
                }
            };
        let user = NewUser {
            name: configuration.openstack.project.clone(),
            openstack_id: configuration.openstack.project_id.clone(),
            project_id: project_id as u32,
            role: 1,
            is_staff: true,
            is_active: true,
        };
        let _user_id = match insert_user_into_db(&mut transaction, &user).await
        {
            Ok(user_id) => user_id,
            Err(MinimalApiError::ValidationError(_)) => {
                tracing::info!("Admin user already exists, skipping.");
                return Ok(());
            }
            Err(MinimalApiError::UnexpectedError(e)) => {
                return Err(e);
            }
        };
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        Ok(())
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
                    .service(hello_scope())
                    .service(user_scope())
                    .service(accounting_scope()),
            )
            .default_service(web::route().to(not_found))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> MySqlPool {
    MySqlPoolOptions::new().connect_lazy_with(configuration.with_db())
}
