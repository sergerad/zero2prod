use tracing_bunyan_formatter::{
    BunyanFormattingLayer, JsonStorageLayer,
};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(
    name: String,
    minimum_severity: String,
) -> anyhow::Result<impl tracing::Subscriber + Send + Sync> {
    // Tracing
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(minimum_severity));
    let formatting_layer =
        BunyanFormattingLayer::new(name, std::io::stdout);
    let registry = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    Ok(registry)
}

pub fn init_subscriber(
    subscriber: impl tracing::Subscriber + Sync + Send,
) -> anyhow::Result<()> {
    LogTracer::init()?;
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
