mod evaluater;
mod tokenizer;
mod tokens;

use crate::evaluater::evaluate;
use crate::tokens::Token;

fn main() {
    let source = "2 3 4 ** ";
    let program: Vec<Token> = tokenizer::tokenize(source).unwrap();
    let result = evaluate(program).unwrap();
    println!("{:?}", result);
}
