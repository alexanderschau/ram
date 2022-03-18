fn main() {
  let example_script = "📣 \"hello world\"
    📣 \"from 🐏!\"";

  println!("🐏 Ram Compiler");

  tokenizer(example_script);
}

struct Token {
  token_type: String,
  value: String,
}

fn tokenizer(script: &str) {
  let mut chars = script.chars();

  let mut tokens: Vec<Token> = Vec::new();

  while let Some(next_char) = chars.next() {
    if next_char == '(' {
      tokens.push(Token {
        token_type: String::from("group"),
        value: String::from("("),
      })
    }

    if next_char == ')' {
      tokens.push(Token {
        token_type: String::from("group"),
        value: String::from(")"),
      })
    }
  }
}
