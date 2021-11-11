mod config;

use anyhow::{anyhow, Result};
use dirs::config_dir;

use crate::config::Replacements;

fn main() -> Result<()> {
    let mut config_path = config_dir().ok_or(anyhow!("Failed to get config dir"))?;
    config_path.push("substitutor");
    config_path.push("config");
    config_path.set_extension("toml");
    let config: Replacements = if config_path.exists() {
        let config_str = std::fs::read_to_string(config_path.as_path())?;
        toml::from_str(&config_str)?
    } else {
        Replacements::default()
    };
    Ok(())
}
