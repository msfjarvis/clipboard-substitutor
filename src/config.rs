use std::str::FromStr;

use anyhow::{bail, Result};
use regex::Regex;
use serde_derive::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct Replacements<'config> {
  #[serde(rename = "substitutor", borrow, default)]
  pub substitutors: Vec<Substitutor<'config>>,
}

#[derive(Debug, Deserialize)]
pub struct Substitutor<'config> {
  #[serde(default)]
  pub name: &'config str,
  #[serde(borrow, alias = "matcher")]
  pub matcher: MatcherType<'config>,
  #[serde(borrow)]
  pub action: Action<'config>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MatcherType<'config> {
  #[serde(borrow)]
  Single(Matcher<'config>),
  #[serde(borrow)]
  Multiple(Vec<Matcher<'config>>),
}

#[derive(Debug, Deserialize)]
pub enum Matcher<'config> {
  #[serde(rename = "starts_with")]
  StartsWith { prefix: &'config str },
  #[serde(rename = "ends_with")]
  EndsWith { suffix: &'config str },
  #[serde(rename = "contains")]
  Contains { substring: &'config str },
  #[serde(rename = "regex")]
  Regex { pattern: &'config str },
  #[serde(rename = "exactly")]
  Exactly { content: &'config str },
}

#[derive(Debug, Deserialize)]
pub enum Action<'config> {
  #[serde(rename = "set")]
  Set { content: &'config str },
  #[serde(rename = "replace")]
  Replace {
    from: &'config str,
    to: &'config str,
  },
  #[serde(rename = "prefix")]
  Prefix { prefix: &'config str },
  #[serde(rename = "suffix")]
  Suffix { suffix: &'config str },
}

pub trait Match {
  fn check_match(&self, string: &str) -> bool;
}

pub trait Act {
  fn apply_action(&self, input: &str) -> String;
}

impl Replacements<'_> {
  pub fn validate(&self) -> Result<()> {
    for subst in self.substitutors.iter() {
      match &subst.matcher {
        MatcherType::Single(matcher) => match matcher {
          Matcher::Regex { pattern } => {
            if let Err(e) = Regex::from_str(pattern) {
              bail!(e);
            }
          }
          _ => {}
        },
        MatcherType::Multiple(matchers) => {
          for matcher in matchers.iter() {
            match matcher {
              Matcher::Regex { pattern } => {
                if let Err(e) = Regex::from_str(pattern) {
                  bail!(e);
                }
              }
              _ => {}
            }
          }
        }
      }
    }
    Ok(())
  }
}

impl Match for Matcher<'_> {
  fn check_match(&self, string: &str) -> bool {
    trace!(?self, ?string, "Checking for match");
    match self {
      Matcher::StartsWith { prefix } => string.starts_with(prefix),
      Matcher::EndsWith { suffix } => string.ends_with(suffix),
      Matcher::Contains { substring } => string.contains(substring),
      Matcher::Regex { pattern } => {
        if let Ok(regex) = Regex::from_str(pattern) {
          regex.is_match(string)
        } else {
          false
        }
      }
      Matcher::Exactly { content } => &string == content,
    }
  }
}

impl Match for MatcherType<'_> {
  fn check_match(&self, string: &str) -> bool {
    match self {
      MatcherType::Single(matcher) => matcher.check_match(string),
      MatcherType::Multiple(matchers) => {
        matchers.iter().all(|matcher| matcher.check_match(string))
      }
    }
  }
}

impl Act for Action<'_> {
  fn apply_action(&self, input: &str) -> String {
    trace!(?self, ?input, "Applying action");
    match self {
      Action::Replace { from, to } => input.replace(from, to),
      Action::Prefix { prefix } => format!("{}{}", prefix, input),
      Action::Suffix { suffix } => format!("{}{}", input, suffix),
      Action::Set { content } => content.to_owned().to_owned(),
    }
  }
}
