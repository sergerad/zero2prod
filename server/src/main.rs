use std::net::TcpListener;

use server::configuration;
use tracing_bunyan_formatter::{
    BunyanFormattingLayer, JsonStorageLayer,
};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Redirect all `log`s events to our subscriber
    LogTracer::init()?;
    // Tracing
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        std::io::stdout,
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    // TCP listener
    let settings = configuration::get_configuration()?;
    let listener = TcpListener::bind(format!(
        "127.0.0.1:{}",
        settings.application_port
    ))?;

    // Read connection string from .env file
    let connection_string = pg::connection_string(".env")?;

    // DB pool
    let pool = pg::get_pool(&connection_string).await?;

    // Server
    let _server = server::run(listener, pool)?.await;

    Ok(())
}
