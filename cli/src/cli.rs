use tracing::info;
use tracing_subscriber::filter::ParseError;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, util::TryInitError, EnvFilter,
};
use types::Error;

pub(super) struct RunCmd {}

/// The default value for the `RUST_LOG` environment variable if one isn't specified otherwise.
const DEFAULT_RUST_LOG: &str = "chain=info,\
     cli=info,\
     warn";

impl RunCmd {
    pub(super) fn run() -> Result<(), Error> {
        init_logging(init_env_filter()?)?;

        info!(target: "cli", "Starting a node...");
        Ok(())
    }
}

fn init_logging(filter: EnvFilter) -> Result<(), TryInitError> {
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .try_init()
}

fn init_env_filter() -> Result<EnvFilter, ParseError> {
    // Parse an `EnvFilter` configuration from the `RUST_LOG`
    // environment variable.
    let v = std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or(DEFAULT_RUST_LOG.to_string());
    EnvFilter::try_new(v)
}
