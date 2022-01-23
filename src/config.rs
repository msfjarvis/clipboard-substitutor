use std::str::FromStr;

use regex::Regex;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Replacements<'config> {
    #[serde(rename = "substitutor", borrow, default)]
    pub substitutors: Vec<Substitutor<'config>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Substitutor<'config> {
    #[serde(default)]
    pub name: &'config str,
    #[serde(borrow)]
    pub matcher: Matcher<'config>,
    #[serde(borrow)]
    pub action: Action<'config>,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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
    fn check_match(self, string: &str) -> bool;
}

pub trait Act {
    fn apply_action(self, input: &str) -> String;
}

impl Match for Matcher<'_> {
    fn check_match(self, string: &str) -> bool {
        match self {
            Matcher::StartsWith { prefix } => string.starts_with(&prefix),
            Matcher::EndsWith { suffix } => string.ends_with(&suffix),
            Matcher::Contains { substring } => string.contains(&substring),
            Matcher::Regex { pattern } => {
                let regex = Regex::from_str(pattern).expect("Failed to parse regex");
                regex.is_match(string)
            }
            Matcher::Exactly { content } => string == content,
        }
    }
}

impl Act for Action<'_> {
    fn apply_action(self, input: &str) -> String {
        return match self {
            Action::Replace { from, to } => input.replace(from, to),
            Action::Prefix { prefix } => format!("{prefix}{input}"),
            Action::Suffix { suffix } => format!("{input}{suffix}"),
            Action::Set { content } => content.to_owned(),
        };
    }
}
