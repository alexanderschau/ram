mod parser;
mod tokenizer;

fn main() {
  let example_script = "📣 \"hello world\"
    📣 \"from 🐏!\"";

  println!("🐏 Ram Compiler");

  let tokens = tokenizer::run(&example_script);
  parser::run(&tokens);
}
