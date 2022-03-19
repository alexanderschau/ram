mod generator;
mod parser;
mod tokenizer;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    let tokens = tokenizer::run(&buffer);
    let ast = parser::run(&tokens);
    let code = generator::rust::gen(&ast);
    println!("{}", code);
    Ok(())
}
