use std::str::FromStr;

use regex::Regex;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Replacements {
    #[serde(rename = "substitutor")]
    pub substitutors: Vec<Substitutor>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Substitutor {
    #[serde(default)]
    pub name: String,
    pub matcher: Matcher,
    pub action: Action,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Matcher {
    #[serde(rename = "starts_with")]
    StartsWith { prefix: String },
    #[serde(rename = "ends_with")]
    EndsWith { suffix: String },
    #[serde(rename = "contains")]
    Contains { substring: String },
    #[serde(rename = "regex")]
    Regex { pattern: String },
}

#[derive(Clone, Debug, Deserialize)]
pub enum Action {
    #[serde(rename = "replace")]
    Replace { from: String, to: String },
    #[serde(rename = "prefix")]
    Prefix { prefix: String },
    #[serde(rename = "suffix")]
    Suffix { suffix: String },
}

pub trait Match {
    fn check_match(self, string: &str) -> bool;
}

pub trait Act {
    fn apply_action(self, input: String) -> String;
}

impl Match for Matcher {
    fn check_match(self, string: &str) -> bool {
        match self {
            Matcher::StartsWith { prefix } => string.starts_with(&prefix),
            Matcher::EndsWith { suffix } => string.ends_with(&suffix),
            Matcher::Contains { substring } => string.contains(&substring),
            Matcher::Regex { pattern } => {
                let regex = Regex::from_str(&pattern).expect("Failed to parse regex");
                regex.is_match(string)
            }
        }
    }
}

impl Act for Action {
    fn apply_action(self, input: String) -> String {
        return match self {
            Action::Replace { from, to } => input.replace(&from, &to),
            Action::Prefix { prefix } => format!("{}{}", prefix, input),
            Action::Suffix { suffix } => format!("{}{}", input, suffix),
        };
    }
}
