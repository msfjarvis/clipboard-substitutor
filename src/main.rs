mod config;
#[cfg(test)]
mod test;

use std::error::Error;
use std::ops::{Deref, Not};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use dirs::config_dir;
use log::{debug, error};

use crate::config::{Act, Match, Replacements};

fn main() -> Result<()> {
    pretty_env_logger::init();
    let config_path = get_config_path()?;
    let config_str = std::fs::read_to_string(config_path.as_path()).unwrap_or_default();
    let config: Replacements<'_> = toml::from_str(&config_str)?;
    if !check_for_version_arg() {
        loop_clipboard(config);
    }
    return Ok(());
}

fn check_for_version_arg() -> bool {
    let args: Vec<String> = std::env::args().collect();
    let version_args = vec!["version", "-v", "--version"];
    for arg in args {
        if version_args.contains(&arg.deref()) {
            print_version();
            return true;
        }
    }
    return false;
}

fn print_version() {
    println!(
        "{}",
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"))
    );
}

fn get_config_path() -> Result<PathBuf> {
    let mut config_path = config_dir().ok_or_else(|| anyhow!("Failed to get config dir"))?;
    config_path.push("substitutor");
    config_path.push("config");
    config_path.set_extension("toml");
    return Ok(config_path);
}

fn loop_clipboard<'a>(config: Replacements<'a>) {
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("Failed to get clipboard");
    let mut clipboard_contents = get_clipboard_contents(&mut clipboard);
    while let Ok(contents) = clipboard_contents.as_deref() {
        if let Some(subst) = config
            .substitutors
            .iter()
            .find(|subst| subst.matcher.clone().check_match(&contents))
        {
            if subst.name.is_empty().not() {
                debug!("{}: matched on {}...", &subst.name, truncate(&contents, 40));
            }
            let result = subst.action.clone().apply_action(contents.deref());
            if let Err(e) = clipboard.set_contents(result.to_owned()) {
                error!("{e}");
            }
        };
        while let Ok(new_contents) = get_clipboard_contents(&mut clipboard) {
            if new_contents != contents {
                clipboard_contents = Ok(new_contents);
                break;
            };
        }
    }
}

fn get_clipboard_contents(clipboard: &mut ClipboardContext) -> Result<String, Box<dyn Error>> {
    clipboard.get_contents()
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
