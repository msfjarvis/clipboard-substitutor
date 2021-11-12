mod config;

use anyhow::{anyhow, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use dirs::config_dir;

use crate::config::{Act, Match, Replacements};

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
    let mut clipboard: ClipboardContext = ClipboardProvider::new().expect("Failed to get clipboard");
    loop {
        let contents = clipboard.get_contents().expect("Failed to read clipboard");
        if let Some(subst) = config
            .substitutors
            .iter()
            .find(|subst| subst.matcher.clone().check_match(&contents))
        {
            let result = subst.action.clone().apply_action(contents);
            let _ = clipboard.set_contents(result);
        };
    }
}
