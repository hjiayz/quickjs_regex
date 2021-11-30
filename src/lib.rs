pub use quickjs_regex_backend::*;
pub use quickjs_regex_derive::*;

#[test]
fn test_regex() {
    let reg = "(Α)123456".repeat(1);
    let text = "α123456".repeat(1000);
    let regex = Regex::compile(&reg, IGNORECASE).unwrap();
    let result = regex.try_match(&text).unwrap();
    assert!(result.len() == 2);

    let reg = "/(\0)123456".repeat(1);
    let text = "/\0123456".repeat(1000);
    let regex = Regex::compile(&reg, IGNORECASE).unwrap();
    let result = regex.try_match(&text).unwrap();
    assert!(result.len() == 2);

    let reg = "(\u{1})(2)(3)456".repeat(1);
    let text = "\u{1}23\u{1}23456\u{1}23";
    let regex = Regex::compile(&reg, IGNORECASE | UNICODE).unwrap();
    let result = regex
        .try_replace(&text, |m| format!("x{}{}{}", m[1], m[2], m[3]))
        .unwrap();
    assert!(result == "\u{1}23x\u{1}23\u{1}23");
    let regex = Regex::compile("(\\d)", UNICODE).unwrap();
    let result = regex
        .try_replacen("12345", |m| format!("x{}", m[1]), 2)
        .unwrap();
    assert!(result == "x1x2345");
    let result = regex
        .try_replace_all("12345", |m| format!("x{}", m[1]))
        .unwrap();
    assert!(result == "x1x2x3x4x5");
    let result = regex.try_match_all("12345").unwrap();
    assert!(result == [["1", "1"], ["2", "2"], ["3", "3"], ["4", "4"], ["5", "5"]]);
}
