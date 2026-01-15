use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init_logging(default_level: Level) -> eyre::Result<()> {
    let stdout_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_line_number(true)
        .with_file(true)
        .with_ansi(true)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(default_level.into())
                .from_env_lossy(), // Setting the RUST_LOG log level environment variable will override the hardcoded version
        );

    Registry::default().with(stdout_layer).init();

    Ok(())
}
