#[cfg(test)]
use crate::tokenizer;

#[test]
fn print_hello_world() {
    let expected = "[Token { token_type: \"name\", value: \"📣\" }, Token { token_type: \"string\", value: \"hello world\" }]";
    assert_eq!(
        format!("{:?}", tokenizer::run("📣 \"hello world\"")),
        expected
    );
}
#[test]
fn print_with_special_char() {
    let expected = "[Token { token_type: \"name\", value: \"📣\" }, Token { token_type: \"string\", value: \"hello world from ram 🐏!\" }]";
    assert_eq!(
        format!("{:?}", tokenizer::run("📣 \"hello world from ram 🐏!\"")),
        expected
    )
}
#[test]
fn print_with_escape() {
    let expected = "[Token { token_type: \"name\", value: \"📣\" }, Token { token_type: \"string\", value: \"hello world\\\\\\\"\" }]";
    assert_eq!(
        format!("{:?}", tokenizer::run("📣 \"hello world\\\"\"")),
        expected
    );
}
#[test]
fn print_addition() {
    let expected = "[Token { token_type: \"name\", value: \"📣\" }, Token { token_type: \"group\", value: \"(\" }, Token { token_type: \"name\", value: \"➕\" }, Token { token_type: \"number\", value: \"39\" }, Token { token_type: \"number\", value: \"2\" }, Token { token_type: \"group\", value: \")\" }]";
    assert_eq!(format!("{:?}", tokenizer::run("📣 (➕ 39 2)")), expected);
}
#[test]
fn print_multiple_values() {
    let expected = "[Token { token_type: \"name\", value: \"📣\" }, Token { token_type: \"string\", value: \"hello world\" }, Token { token_type: \"group\", value: \"(\" }, Token { token_type: \"name\", value: \"➕\" }, Token { token_type: \"number\", value: \"39\" }, Token { token_type: \"number\", value: \"2\" }, Token { token_type: \"group\", value: \")\" }, Token { token_type: \"string\", value: \"third message\" }]";
    assert_eq!(
        format!(
            "{:?}",
            tokenizer::run("📣 \"hello world\" (➕ 39 2) \"third message\"")
        ),
        expected
    );
}
