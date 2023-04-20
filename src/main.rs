use std::io::Write;

use crate::token::{evaluate, tokenize, ValueToken};
mod token;

fn print_result(result: Vec<ValueToken>) {
    for value in result {
        print!("{} ", value);
    }
    println!();
}

fn main() -> std::io::Result<()> {
    'a: loop {
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input)?;
        if input == "quit" {
            break 'a;
        }

        let mut tokens = tokenize(&input).unwrap();
        let result = evaluate(&mut tokens).unwrap();
        print_result(result);
    }
    Ok(())
}
