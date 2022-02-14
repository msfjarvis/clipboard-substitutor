use crate::config::{Match, Matcher};

#[test]
fn regex_matcher() {
    let matcher = Matcher::Regex {
        pattern: "^https.*",
    };
    assert!(matcher.check_match("https://example.com"));
    assert!(!matcher.check_match("example.com"));
}
