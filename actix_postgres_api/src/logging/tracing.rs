use std::io;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::*, registry::Registry, EnvFilter,
};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // Dodajemy ograniczenie Clone dla parametru generycznego Sink
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static + Clone,
{
    // We're falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Create a formatting layer for console output
    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_writer(sink.clone());

    // Create the JSON Bunyan formatting layer for structured logging
    let bunyan_formatting_layer = BunyanFormattingLayer::new(name, sink);

    // Create our Subscriber
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(bunyan_formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // Register our subscriber as the global default
    set_global_default(subscriber).expect("Failed to set subscriber");
}

/// Initialize structured logging with log level from environment or default to specified level
pub fn init_logging(app_name: &str, default_level: &str) {
    let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| format!("{app_name}={default_level},actix_web=info,sqlx=warn"));
    
    let subscriber = get_subscriber(app_name.into(), env_filter, io::stdout);
    init_subscriber(subscriber);
    
    tracing::info!("Initialized logging system with tracing");
}

/// Create a database query span which tracks performance
pub fn create_db_span(
    query_name: &'static str,
    sql: &'static str,
    params: &str,
) -> tracing::Span {
    tracing::info_span!(
        "database_query",
        %query_name,
        %sql,
        params = %params,
    )
}