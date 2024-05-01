use std::net::TcpListener;

use server::configuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TCP listener
    let settings = configuration::get_configuration()?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))?;

    // Read connection string from .env file
    let connection_string = pg::connection_string(".env")?;

    // DB pool
    let pool = pg::get_pool(&connection_string).await?;

    // Server
    let _server = server::run(listener, pool)?.await;

    Ok(())
}
