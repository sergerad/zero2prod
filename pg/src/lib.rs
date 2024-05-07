use log::info;
use secrecy::{ExposeSecret, Secret};
use std::{path::PathBuf, sync::mpsc::channel};

use sqlx::PgPool;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;

pub async fn spawn_and_wait() -> anyhow::Result<()> {
    // Read settings from config file
    let settings = conf::get_configuration(None)?;

    // Start Postgres container
    let (node, pool) = spawn_pg(&settings.database).await?;
    info!("Container is running. Waiting for signal to stop.");

    // Close connection pool
    pool.close().await;

    // Overwrite local configuration
    let updated_settings = conf::Settings {
        database: conf::DatabaseSettings {
            port: node.get_host_port_ipv4(5432).await,
            ..settings.database
        },
        ..settings
    };
    let base_config_file = conf::base_config_file()?;
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(false)
        .open(base_config_file)?;
    serde_yaml::to_writer(f, &updated_settings).unwrap();
    // Store connection string in .env file
    info!("DB port written to configuration file");

    // Store connection string in .env file
    let connection_string = updated_settings
        .database
        .connection_string(updated_settings.database.port);
    let mut value = serde_envfile::Value::new();
    value.insert(
        "DATABASE_URL".into(),
        connection_string.expose_secret().into(),
    );
    serde_envfile::to_file(&PathBuf::from(".env"), &value)?;
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
    settings: &conf::DatabaseSettings,
) -> anyhow::Result<(ContainerAsync<Postgres>, PgPool)> {
    // Start Postgres container
    let node = Postgres::default()
        .with_db_name(&settings.database_name)
        .with_password(settings.password.expose_secret())
        .with_user(&settings.username)
        .start()
        .await;

    // Construct connections
    let host_port = node.get_host_port_ipv4(5432).await;
    let pool = PgPool::connect(
        settings
            .connection_string(host_port)
            .expose_secret()
            .as_str(),
    )
    .await?;

    // Run migrations
    sqlx::migrate!("../migrations").run(&pool).await?;

    // Return container and pool
    Ok((node, pool))
}

pub async fn migrate_pg() -> anyhow::Result<()> {
    // Create pool
    let connection_string = connection_string(".env")?;
    let pool = PgPool::connect(connection_string.expose_secret()).await?;

    // Run migrations
    sqlx::migrate!("../migrations").run(&pool).await?;

    Ok(())
}

pub fn connection_string(env_file_path: &str) -> anyhow::Result<Secret<String>> {
    // Read .env file and return content
    let env_file: conf::EnvFile = serde_envfile::from_file(&PathBuf::from(env_file_path))?;
    Ok(env_file.database_url)
}
