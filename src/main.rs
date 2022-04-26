mod clipboard;
mod config;
#[cfg(test)]
mod test;

use std::ops::Deref;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use dirs::config_dir;

use crate::clipboard::monitor_clipboard;
use crate::config::Replacements;

const VERSION_ARGS: [&str; 3] = ["version", "-v", "--version"];

fn main() -> Result<()> {
  if check_for_version_arg() {
    return Ok(());
  }
  configure_tracing();
  let config_path = get_config_path()?;
  let config_str =
    std::fs::read_to_string(config_path.as_path()).unwrap_or_default();
  let config: Replacements<'_> = toml::from_str(&config_str)?;
  monitor_clipboard(config);
  Ok(())
}

fn check_for_version_arg() -> bool {
  let args: Vec<String> = std::env::args().collect();
  for arg in args {
    if VERSION_ARGS.contains(&arg.deref()) {
      print_version();
      return true;
    }
  }
  false
}

fn print_version() {
  println!(
    "{}",
    concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"))
  );
}

#[cfg(not(feature = "journald"))]
fn configure_tracing() {
  use tracing::Level;
  use tracing_subscriber::FmtSubscriber;

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::TRACE)
    .finish();

  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
}

#[cfg(feature = "journald")]
fn configure_tracing() {
  use tracing_journald::Layer;
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::Registry;

  let subscriber =
    Registry::default().with(Layer::new().unwrap().with_field_prefix(None));
  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
}

fn get_config_path() -> Result<PathBuf> {
  let mut config_path =
    config_dir().ok_or_else(|| anyhow!("Failed to get config dir"))?;
  config_path.push("substitutor");
  config_path.push("config");
  config_path.set_extension("toml");
  Ok(config_path)
}
