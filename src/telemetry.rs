use tracing::{dispatcher::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a subscriber
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // Get filter level from RUST_LOG, or use INFO as default
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // Output the formatted span into stdout
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);

    Registry::default()
        .with(env_filter) // extent subscriber
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register subscriber globally to process span data
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all logs to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // set the subscriber for the application
    set_global_default(subscriber.into()).expect("Failed to set tracing subscriber");
}
