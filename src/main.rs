mod generator;
mod parser;
mod tokenizer;

fn main() {
  let example_script = "📣 \"hello world\"
    📣 \"from 🐏!\"";

  println!("🐏 Ram Compiler");

  let tokens = tokenizer::run(&example_script);
  println!("Tokens: {:?}", tokens);
  let ast = parser::run(&tokens);
  println!("AST: {:?}", ast);

  let code = generator::rust::gen(&ast);
  println!("Code: {}", code);
}
