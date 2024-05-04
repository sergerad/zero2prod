use secrecy::ExposeSecret;
use server::configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Tracing
    let subscriber =
        server::trace::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout)?;
    server::trace::init_subscriber(subscriber)?;

    // TCP listener
    let settings = configuration::get_configuration()?;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.application_port))?;

    // Read connection string from .env file
    let connection_string = pg::connection_string(".env")?;

    // DB pool
    let pool = pg::get_pool(connection_string.expose_secret()).await?;

    // Server
    let _server = server::run(listener, pool)?.await;

    Ok(())
}
