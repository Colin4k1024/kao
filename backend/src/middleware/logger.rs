use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger(rust_log: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| rust_log.parse().unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
