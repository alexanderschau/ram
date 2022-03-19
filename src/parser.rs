use crate::tokenizer;

#[derive(Debug)]
pub struct CallExpressionNode {
  pub name: String,
  pub params: Vec<Node>,
}

#[derive(Debug)]
pub enum Node {
  NumberLiteral(String),
  StringLiteral(String),
  CallExpression(CallExpressionNode),
}

#[derive(Debug)]
pub struct Program {
  pub body: Vec<Node>,
}

struct ProgramBuilder<'a> {
  tokens: &'a Vec<tokenizer::Token>,
}
impl ProgramBuilder<'_> {
  fn walk(&mut self, input_current: &usize) -> (Node, usize) {
    // let mut current = self.current;
    let mut current = *input_current;
    let token = &self.tokens[current];

    if token.token_type == "string" {
      return (Node::StringLiteral(token.value.clone()), current);
    }

    if token.token_type == "number" {
      return (Node::NumberLiteral(token.value.clone()), current);
    }

    if token.token_type == "name" {
      let mut node = CallExpressionNode {
        name: token.value.clone(),
        params: Vec::new(),
      };

      // get all parameters
      current += 1;
      while current < self.tokens.len() {
        let (sub_node, sub_current) = self.walk(&current);
        if let Node::CallExpression(_) = sub_node {
          current -= 1;
          break;
        }
        node.params.push(sub_node);
        current = sub_current;

        current += 1;
      }

      return (Node::CallExpression(node), current);
    }

    (Node::StringLiteral(String::from("asd")), current)
  }
}

pub fn run(tokens: &Vec<tokenizer::Token>) -> Program {
  let mut program_builder = ProgramBuilder { tokens: tokens };

  let mut program = Program { body: Vec::new() };

  let mut current = 0;

  while current < program_builder.tokens.len() {
    let (node, node_current) = program_builder.walk(&current);
    program.body.push(node);

    current = node_current + 1;
  }

  program
}
