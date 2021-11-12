use quickjs_regex_derive::uregex;

#[test]
fn test_uregex(){

    const TEST123 : quickjs_regex::Regex = uregex!("123");
    assert!(TEST123.test("123"));
}
