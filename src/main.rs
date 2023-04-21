mod tokens;
mod tokenizer;
mod evaluater;
use std::io::Write;

fn print_result(result: Vec<tokens::ValueToken>) {
    for value in result {
        print!("{} ", value);
    }
    println!();
}

fn main() -> std::io::Result<()> {
    'a: loop {
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut input)?;
        if input == "quit" {
            break 'a;
        }

        let mut tokens = tokenizer::tokenize(&input).unwrap();
        let result = evaluater::evaluate(&mut tokens).unwrap();
        print_result(result);
    }
    Ok(())
}
