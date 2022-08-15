use tracing::dispatcher::SetGlobalDefaultError;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::filter::Targets;

#[cfg(not(feature = "journald"))]
fn configure_tracing(filter: Targets) -> Result<(), SetGlobalDefaultError> {
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::{fmt, Layer};

  let stdout_log = fmt::layer().pretty();
  let subscriber =
    tracing_subscriber::registry().with(stdout_log.with_filter(filter));
  set_global_default(subscriber)
}

#[cfg(feature = "journald")]
fn configure_tracing(filter: Targets) -> Result<(), SetGlobalDefaultError> {
  use tracing_journald::layer;
  use tracing_subscriber::{layer::SubscriberExt, registry};

  let subscriber = registry().with(filter).with(layer().unwrap());
  set_global_default(subscriber)
}

pub fn init() -> Result<(), SetGlobalDefaultError> {
  let tracing_filter =
    Targets::new().with_target("clipboard_subsitutor", Level::DEBUG);
  configure_tracing(tracing_filter)
}
