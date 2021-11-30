use quickjs_regex_derive::{regex, uregex};

#[test]
fn test_regex() {
    const TEST123: quickjs_regex::Regex = uregex!("123");
    assert!(TEST123.test("123"));
    assert!(regex!(r#"\u{123}"#, "u").test("\u{123}"));
}
