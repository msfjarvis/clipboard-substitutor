use crate::config::{Act, Action, Match, Matcher};

#[test]
fn regex_matcher() {
    let matcher = Matcher::Regex {
        pattern: "^https.*",
    };
    assert!(matcher.check_match("https://example.com"));
    assert!(!matcher.check_match("example.com"));
}

#[test]
fn set_action() {
    let action = Action::Set { content: "doe" };
    assert_eq!("doe", action.apply_action("john"));
}

#[test]
fn replace_action() {
    let action = Action::Replace {
        from: "doe",
        to: "bow",
    };
    assert_eq!("john bow", action.apply_action("john doe"));
}

#[test]
fn prefix_action() {
    let action = Action::Prefix { prefix: "hello " };
    assert_eq!("hello john", action.apply_action("john"));
}

#[test]
fn suffix_action() {
    let action = Action::Suffix { suffix: " doe" };
    assert_eq!("john doe", action.apply_action("john"));
}
