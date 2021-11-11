#![allow(dead_code)]
use serde_derive::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Replacements {
    #[serde(rename = "substitutor")]
    pub substitutors: Vec<Substitutor>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Substitutor {
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
    #[serde(rename = "remove")]
    Remove { substring: String },
}
