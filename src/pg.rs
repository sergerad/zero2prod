use sqlx::PgPool;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;

use crate::configuration::DatabaseSettings;

pub async fn run_pg(
    settings: &DatabaseSettings,
) -> anyhow::Result<(ContainerAsync<Postgres>, PgPool)> {
    let node = Postgres::default()
        .with_db_name(&settings.database)
        .with_password(&settings.password)
        .with_user(&settings.user)
        .start()
        .await;
    let host_port = node.get_host_port_ipv4(5432).await;
    let pool = PgPool::connect(settings.connection_string(host_port).as_str()).await?;

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    Ok((node, pool))
}
