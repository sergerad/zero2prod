use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use tracing_subscriber::fmt::MakeWriter;

pub fn get_subscriber<Sink>(
    name: String,
    minimum_severity: String,
    sink: Sink,
) -> anyhow::Result<impl tracing::Subscriber + Send + Sync>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Tracing
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(minimum_severity));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    let registry = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    Ok(registry)
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Sync + Send) -> anyhow::Result<()> {
    LogTracer::init()?;
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
