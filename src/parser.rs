use crate::tokenizer;

pub struct CallExpressionNode {
  name: String,
  params: Vec<Node>,
}

pub enum Node {
  NumberLiteral(String),
  StringLiteral(String),
  CallExpression(CallExpressionNode),
}

pub struct Program {
  body: Vec<Node>
}

pub fn run(tokens: &Vec<tokenizer::Token>) -> Program {
  let mut current = 0;

  let mut program = Program{
    body: Vec::new()
  };

  fn walk() {}

  while current < tokens.len() {
    println!("{:?}", tokens[current]);

    current += 1;
  };

  program
}
