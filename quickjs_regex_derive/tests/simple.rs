use quickjs_regex_derive::uregex;

#[test]
fn test_uregex(){
    assert!(uregex!("123").test("123"));
}
