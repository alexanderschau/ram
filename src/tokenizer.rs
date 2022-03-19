use regex::Regex;

#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
}

pub fn run(script: &str) -> Vec<Token> {
    let mut chars = script.chars();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(next_char) = chars.next() {
        // group
        if next_char == '(' {
            tokens.push(Token {
                token_type: String::from("group"),
                value: String::from("("),
            });
            continue;
        }

        if next_char == ')' {
            tokens.push(Token {
                token_type: String::from("group"),
                value: String::from(")"),
            });
            continue;
        }

        // name
        if "📣🕹️📥📤💾➕".contains(next_char) {
            tokens.push(Token {
                token_type: String::from("name"),
                value: String::from(next_char),
            });
            continue;
        }

        // variable
        let var_exp = Regex::new(r"(?i)[a-zA-Z]").unwrap();
        let result = String::from(next_char);
        if var_exp.is_match(&result) {
            let mut value = result.to_owned();

            let var_mid_exp = Regex::new(r"(?i)[a-zA-Z_0-9]").unwrap();
            while let Some(next_char) = chars.next() {
                let result = String::from(next_char);
                if var_mid_exp.is_match(&result) {
                    value.push_str(&result);
                    continue;
                }
                break;
            }

            tokens.push(Token {
                token_type: String::from("variable"),
                value: value,
            });
            continue;
        }

        // number
        if next_char.is_numeric() {
            let mut value = String::from(next_char).to_owned();

            while let Some(next_char) = chars.next() {
                if next_char.is_numeric() {
                    value.push_str(&String::from(next_char));
                    continue;
                }
                break;
            }

            tokens.push(Token {
                token_type: String::from("number"),
                value: value,
            });
            continue;
        }

        // string
        if next_char == '"' {
            let mut value = String::new().to_owned();

            while let Some(next_char) = chars.next() {
                if next_char == '"' {
                    break;
                }
                value.push_str(&String::from(next_char));
            }

            tokens.push(Token {
                token_type: String::from("string"),
                value: value,
            });
            continue;
        }
    }

    tokens
}
