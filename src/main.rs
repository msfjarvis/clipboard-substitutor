mod clipboard;
mod config;
mod logging;
#[cfg(test)]
mod test;

use std::path::PathBuf;

use anyhow::{Result, anyhow, bail};
use dirs::config_dir;
use tracing::debug;

use crate::clipboard::monitor;
use crate::config::Replacements;

fn main() -> Result<()> {
  if check_for_version_arg() {
    return Ok(());
  }
  if let Err(e) = logging::init() {
    bail!(e)
  };
  let config_path = get_config_path()?;
  let config_str =
    std::fs::read_to_string(config_path.as_path()).unwrap_or_default();
  let config: Replacements = toml::from_str(&config_str)?;
  config.validate()?;
  monitor(&config)
}

fn check_for_version_arg() -> bool {
  for arg in argv::iter() {
    if arg == "-v" || arg == "version" || arg == "--version" {
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

fn get_config_path() -> Result<PathBuf> {
  let mut config_path =
    config_dir().ok_or_else(|| anyhow!("Failed to get config dir"))?;
  config_path.push("substitutor");
  config_path.push("config");
  config_path.set_extension("toml");
  debug!("Config file: {}", config_path.to_string_lossy());
  Ok(config_path)
}
