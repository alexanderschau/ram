use crate::tokenizer;

struct CallExpressionNode {
  name: String,
  params: Vec<String>,
}

enum Node {
  NumberLiteral(String),
  StringLiteral(String),
  CallExpression(CallExpressionNode),
}

pub fn run(tokens: &Vec<tokenizer::Token>) {
  let mut current = 0;

  fn walk() {}

  while current < tokens.len() {
    println!("{:?}", tokens[current]);

    current += 1;
  }
}
