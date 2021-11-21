mod config;

use std::ops::Not;

use anyhow::{anyhow, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use dirs::config_dir;
use log::{debug, error};

use crate::config::{Act, Match, Replacements};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let mut config_path = config_dir().ok_or_else(|| anyhow!("Failed to get config dir"))?;
    config_path.push("substitutor");
    config_path.push("config");
    config_path.set_extension("toml");
    let config: Replacements = if config_path.exists() {
        let config_str = std::fs::read_to_string(config_path.as_path())?;
        toml::from_str(&config_str)?
    } else {
        Replacements::default()
    };
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("Failed to get clipboard");
    loop {
        let contents = clipboard.get_contents().expect("Failed to read clipboard");
        if let Some(subst) = config
            .substitutors
            .iter()
            .find(|subst| subst.matcher.clone().check_match(&contents))
        {
            if subst.name.is_empty().not() {
                debug!("{}: matched on {}...", &subst.name, truncate(&contents, 40));
            }
            let result = subst.action.clone().apply_action(contents);
            if let Err(e) = clipboard.set_contents(result) {
                error!("{}", e);
            }
        };
    }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
