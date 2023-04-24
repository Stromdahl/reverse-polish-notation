mod evaluater;
mod tokenizer;
mod tokens;

use crate::evaluater::evaluate;
use crate::tokens::Token;

fn main() {
    let x = 4;
    let y = 3;
    let program: Vec<Token> = vec![Token::Push(x), Token::Push(y), Token::Add];
    assert_eq!(vec![x + y], evaluate(program).unwrap());
}
