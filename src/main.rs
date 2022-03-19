mod generator;
mod parser;
mod tokenizer;

fn main() {
  let example_script = "📣 \"hello world\" \" lol \"
    📣 (➕ 3 5 )
    📣 \"from 🐏!\"";

  // println!("🐏 Ram Compiler");

  let tokens = tokenizer::run(&example_script);
  println!("{:?}", tokens);
  let ast = parser::run(&tokens);
  println!("{:?}", ast);
  let code = generator::rust::gen(&ast);
  println!("{}", code);
}
