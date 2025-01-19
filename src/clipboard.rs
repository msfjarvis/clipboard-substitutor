use crate::config::{Act, Match, Replacements};
use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::time::Duration;
use tracing::{debug, error};

pub fn monitor(config: &Replacements) -> Result<()> {
    loop {
        let mut clipboard = ClipboardContext::new().expect("Failed to get clipboard");
        if let Ok(contents) = clipboard.get_contents() {
            if let Some(subst) = config
                .substitutors
                .iter()
                .find(|subst| subst.matcher.check_match(&contents))
            {
                if !subst.name.is_empty() {
                    debug!(?subst.name, ?contents);
                }
                let result = subst.action.apply_action(&contents);
                if let Err(e) = clipboard.set_contents(result) {
                    error!("{}", e);
                }
            };
        }
        std::thread::sleep(Duration::from_millis(1_000));
    }
}
