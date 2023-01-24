use std::io;
use std::ops::Not;

use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use copypasta::{ClipboardContext, ClipboardProvider};
use tracing::{debug, error};

use crate::config::{Act, Match, Replacements};

struct Handler {
  ctx: ClipboardContext,
  config: Replacements,
}

impl ClipboardHandler for Handler {
  fn on_clipboard_change(&mut self) -> CallbackResult {
    if let Ok(contents) = self.ctx.get_contents() {
      if let Some(subst) = self
        .config
        .substitutors
        .iter()
        .find(|subst| subst.matcher.check_match(&contents))
      {
        if subst.name.is_empty().not() {
          debug!(?subst.name, ?contents);
        }
        let result = subst.action.apply_action(&contents);
        if let Err(e) = self.ctx.set_contents(result) {
          error!("{}", e);
        }
      };
    }
    CallbackResult::Next
  }

  fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
    error!("Error: {}", error);
    CallbackResult::Next
  }
}

pub fn monitor(config: Replacements) {
  let ctx = ClipboardContext::new().expect("Failed to acquire clipboard");
  let handler = Handler { ctx, config };
  let _master = Master::new(handler).run();
}
