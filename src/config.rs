use std::str::FromStr;

use anyhow::{Result, bail};
use regex::Regex;
use serde_derive::Deserialize;
use tracing::trace;

#[derive(Debug, Deserialize)]
pub struct Replacements {
  #[serde(rename = "substitutor", default)]
  pub substitutors: Vec<Substitutor>,
}

#[derive(Debug, Deserialize)]
pub struct Substitutor {
  #[serde(default)]
  pub name: String,
  #[serde(alias = "matcher")]
  pub matcher: MatcherType,
  pub action: Action,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MatcherType {
  Single(Matcher),
  Multiple(Vec<Matcher>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Matcher {
  StartsWith { prefix: String },
  EndsWith { suffix: String },
  Contains { substring: String },
  Regex { pattern: String },
  Exactly { content: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
  Set { content: String },
  Replace { from: String, to: String },
  Prefix { prefix: String },
  Suffix { suffix: String },
}

pub trait Match {
  fn check_match(&self, string: &str) -> bool;
}

pub trait Act {
  fn apply_action(&self, input: &str) -> String;
}

impl Replacements {
  pub fn validate(&self) -> Result<()> {
    for subst in &self.substitutors {
      match &subst.matcher {
        MatcherType::Single(matcher) => {
          if let Matcher::Regex { pattern } = matcher {
            if let Err(e) = Regex::from_str(pattern) {
              bail!(e);
            }
          }
        }
        MatcherType::Multiple(matchers) => {
          for matcher in matchers {
            if let Matcher::Regex { pattern } = matcher {
              if let Err(e) = Regex::from_str(pattern) {
                bail!(e);
              }
            }
          }
        }
      }
    }
    Ok(())
  }
}

impl Match for Matcher {
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
      Matcher::Exactly { content } => string == content,
    }
  }
}

impl Match for MatcherType {
  fn check_match(&self, string: &str) -> bool {
    match self {
      MatcherType::Single(matcher) => matcher.check_match(string),
      MatcherType::Multiple(matchers) => {
        matchers.iter().all(|matcher| matcher.check_match(string))
      }
    }
  }
}

impl Act for Action {
  fn apply_action(&self, input: &str) -> String {
    trace!(?self, ?input, "Applying action");
    match self {
      Action::Replace { from, to } => input.replace(from, to),
      Action::Prefix { prefix } => format!("{prefix}{input}"),
      Action::Suffix { suffix } => format!("{input}{suffix}"),
      Action::Set { content } => content.clone(),
    }
  }
}
