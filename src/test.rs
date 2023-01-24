use assay::assay;

use crate::config::{Act, Action, Match, Matcher, MatcherType, Replacements};

#[assay]
fn regex_matcher() {
  let matcher = Matcher::Regex {
    pattern: "^https.*".to_string(),
  };
  assert!(matcher.check_match("https://example.com"));
  assert!(!matcher.check_match("example.com"));
}

#[assay]
fn set_action() {
  let action = Action::Set {
    content: "doe".to_string(),
  };
  assert_eq!("doe", &action.apply_action("john"));
}

#[assay]
fn replace_action() {
  let action = Action::Replace {
    from: "doe".to_string(),
    to: "bow".to_string(),
  };
  assert_eq!("john bow", &action.apply_action("john doe"));
}

#[assay]
fn prefix_action() {
  let action = Action::Prefix {
    prefix: "hello ".to_string(),
  };
  assert_eq!("hello john", &action.apply_action("john"));
}

#[assay]
fn suffix_action() {
  let action = Action::Suffix {
    suffix: " doe".to_string(),
  };
  assert_eq!("john doe", &action.apply_action("john"));
}

#[assay]
fn parse_with_multiple_matchers() {
  let config = r#"
    [[substitutor]]
    name = "Example"
    matcher = [
        { starts_with = { prefix = "https://example.com" } },
        { ends_with = { suffix = ".mp4" } }
    ]
    action = { prefix = { prefix = "/mirror" } }
    "#;
  let config: Replacements = toml::from_str(config)?;
  assert_eq!(1, config.substitutors.len());
  let subst = &config.substitutors[0];
  assert_eq!("Example", &subst.name);
  assert!(matches!(subst.matcher, MatcherType::Multiple(_)));
  assert!(matches!(subst.action, Action::Prefix { .. }));
}

#[assay]
fn parse_with_single_matcher() {
  let config = r#"
    [[substitutor]]
    name = "Example"
    matcher = { starts_with = { prefix = "https://example.com" } }
    action = { prefix = { prefix = "/mirror" } }
    "#;
  let config: Replacements = toml::from_str(config)?;
  assert_eq!(1, config.substitutors.len());
  let subst = &config.substitutors[0];
  assert_eq!("Example", &subst.name);
  assert!(matches!(subst.matcher, MatcherType::Single(_)));
  assert!(matches!(subst.action, Action::Prefix { .. }));
}

#[assay]
fn config_validation_success() {
  let config = r#"
[[substitutor]]
name = "vxTwitter"
matcher = { regex = { pattern = "^https://(?P<host>(?:mobile.)?twitter.com)/.*/status/[0-9]+.*" } }
action = { replace = { from = "twitter.com", to = "vxtwitter.com" } }
  "#;
  let config: Replacements = toml::from_str(config)?;
  assert!(matches!(config.validate(), Ok(_)));
}

#[assay]
fn config_validation_failure() {
  let config = r#"
[[substitutor]]
name = "vxTwitter"
matcher = { regex = { pattern = "^https://(?P<>(?:mobile.)?twitter.com)/.*/status/[0-9]+.*" } }
action = { replace = { from = "twitter.com", to = "vxtwitter.com" } }
  "#;
  let config: Replacements = toml::from_str(config)?;
  assert!(matches!(config.validate(), Err(_)));
}
