mod generator;
mod parser;
mod tokenizer;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let mut result_lang = String::from("js");
    if args.len() >= 2 {
        let res = args.nth(1).unwrap();
        result_lang = res;
    }
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let tokens = tokenizer::run(&buffer);
    //println!("{:?}", tokens);
    let ast = parser::run(&tokens);
    //println!("{:?}", ast);
    let code = match result_lang.as_str() {
        "js" => generator::javascript::gen(&ast),
        _ => generator::rust::gen(&ast),
    };
    println!("{}", code);
    Ok(())
}