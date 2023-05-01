mod evaluater;
mod tokenizer;
mod tokens;

use crate::evaluater::VirtualMachine;
use crate::tokens::Token;

fn main() {
    let mut vm: VirtualMachine = VirtualMachine::new();
    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "quit" {
            break;
        }
        let source = input.trim().to_string();
        let program: Vec<Token> = tokenizer::Lexer::new(&source).collect();
        match vm.evaluate(program) {
            Ok(..) => {
                println!("{:?}", vm.stack);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
