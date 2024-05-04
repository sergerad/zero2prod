use log::info;
use std::{fs, sync::mpsc::channel};

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

pub async fn spawn_and_wait() -> anyhow::Result<()> {
    // Read settings from config file
    let settings = get_configuration()?.database;

    // Start Postgres container
    let (node, pool) = spawn_pg(&settings).await?;
    info!("Container is running. Waiting for signal to stop.");

    // Close connection pool
    pool.close().await;

    // Store connection string in .env file
    let connection_string = settings.connection_string(node.get_host_port_ipv4(5432).await);
    fs::write(".env", format!("DATABASE_URL=\"{connection_string}\"",))?;
    info!("Connection string written to .env");

    // Listen for signal to stop
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))?;
    rx.recv()?;

    // Shut container down
    info!("Shutting down");
    node.stop().await;
    Ok(())
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

pub async fn migrate_pg() -> anyhow::Result<()> {
    // Create pool
    let connection_string = connection_string(".env")?;
    let pool = PgPool::connect(&connection_string).await?;

    // Run migrations
    sqlx::migrate!("../db/migrations").run(&pool).await?;

    Ok(())
}

pub fn connection_string(env_file_path: &str) -> anyhow::Result<String> {
    // Read .env file and return content
    let connection_string = fs::read_to_string(env_file_path)?
        .trim_start_matches("DATABASE_URL=")
        .replace('"', "")
        .to_string();
    Ok(connection_string)
}

pub fn replace_db(connection_string: String, db_name: &str) -> anyhow::Result<String> {
    match connection_string.rsplit_once('/') {
        Some((prefix, _)) => Ok(format!("{}/{}", prefix, db_name)),
        None => Err(anyhow::anyhow!("Invalid connection string format.")),
    }
}

pub async fn get_pool(connection_string: &str) -> anyhow::Result<PgPool> {
    let pool = PgPool::connect(connection_string).await?;
    Ok(pool)
}
