use std::fs;

use sqlx::PgPool;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self, port: u16) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, port, self.database,
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize()
}

pub async fn spawn_pg(
    settings: &DatabaseSettings,
) -> anyhow::Result<(ContainerAsync<Postgres>, PgPool)> {
    // Start Postgres container
    let node = Postgres::default()
        .with_db_name(&settings.database)
        .with_password(&settings.password)
        .with_user(&settings.user)
        .start()
        .await;

    // Construct connections
    let host_port = node.get_host_port_ipv4(5432).await;
    let pool = PgPool::connect(settings.connection_string(host_port).as_str()).await?;

    // Run migrations
    sqlx::migrate!("../db/migrations").run(&pool).await?;

    // Return container and pool
    Ok((node, pool))
}

fn connection_string() -> anyhow::Result<String> {
    // Read .env file and return content
    let connection_string = fs::read_to_string(".env")?
        .trim_start_matches("DATABASE_URL=")
        .replace('"', "")
        .to_string();
    Ok(connection_string)
}

pub async fn get_pool() -> anyhow::Result<PgPool> {
    // Read connection string from .env file
    let connection_string = connection_string()?;

    // Construct pool
    let pool = PgPool::connect(&connection_string).await?;
    Ok(pool)
}
