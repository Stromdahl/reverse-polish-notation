mod evaluater;
mod tokenizer;
mod tokens;

use crate::evaluater::evaluate;
use crate::tokens::Token;

fn main() {
    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "quit" {
            break;
        }
        let program: Vec<Token> = tokenizer::tokenize(input.trim()).unwrap();
        let result = evaluate(program);
        println!("{:?}", result);
    }
}
