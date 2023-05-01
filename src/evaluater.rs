use crate::tokens::Token;
use std::collections::HashMap;

pub struct VirtualMachine {
    pub stack: Vec<i32>,
    variables: HashMap<String, i32>,
}

fn try_pop_stack(stack: &mut Vec<i32>) -> Result<i32, &'static str> {
    Ok(stack.pop().ok_or("Syntax Error")?)
}

fn try_pop_operators(stack: &mut Vec<i32>) -> Result<(i32, i32), &'static str> {
    Ok((try_pop_stack(stack)?, try_pop_stack(stack)?))
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            variables: HashMap::new(),
        }
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.stack);
    }

    pub fn evaluate(&mut self, program: Vec<Token>) -> Result<(), &'static str> {
        for token in program {
            match token {
                Token::Push(x) => self.stack.push(x),
                Token::Pop => {self.stack.pop();},
                Token::Add => {
                    let (right, left) = try_pop_operators(&mut self.stack)?;
                    self.stack.push(left + right);
                }
                Token::Sub => {
                    let (right, left) = try_pop_operators(&mut self.stack)?;
                    self.stack.push(left - right);
                }
                Token::Mul => {
                    let (right, left) = try_pop_operators(&mut self.stack)?;
                    self.stack.push(left * right);
                }
                Token::Div => {
                    let (right, left) = try_pop_operators(&mut self.stack)?;
                    self.stack.push(left / right);
                }
                Token::Load(name) => {
                    let value = match self.variables.get(name) {
                        Some(value) => value,
                        None => &0,
                    };
                    self.stack.push(*value);
                }
                Token::Store(name) => {
                    let value = try_pop_stack(&mut self.stack)?;
                    self.variables.insert(name.to_string(), value);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_add_variable_to_variable() {
        let key = "x";
        let program: Vec<Token> = vec![
            Token::Push(13),
            Token::Store(key),
            Token::Load(key),
            Token::Load(key),
            Token::Add,
        ];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 26);
    }

    #[test]
    fn test_evaluate_add_variable_to_literal() {
        let key = "x";
        let program: Vec<Token> = vec![
            Token::Push(13),
            Token::Store(key),
            Token::Load(key),
            Token::Push(7),
            Token::Add,
        ];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 20);
    }

    #[test]
    fn test_evaluate_assign() {
        let key = "x";
        let program: Vec<Token> = vec![
            Token::Push(13),
            Token::Store(key),
        ];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!( *vm.variables.get(&key.to_string()).unwrap(), 13);
    }

    #[test]
    fn test_evaluate_variable() {
        let key = "x";
        let program: Vec<Token> = vec![Token::Load(key)];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_evaluate_pop() {
        let program: Vec<Token> = vec![Token::Push(5), Token::Push(4), Token::Pop];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vec![5], vm.stack);
    }

    #[test]
    fn test_evaluate() {
        let x = 4;
        let y = 3;
        let program: Vec<Token> = vec![Token::Push(x), Token::Push(y), Token::Add];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vec![x + y], vm.stack);
    }

    #[test]
    fn test_evaluate_sub() {
        let program: Vec<Token> = vec![Token::Push(3), Token::Push(4), Token::Sub];
        let expected = vec![-1];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack, expected);
    }

    #[test]
    fn test_evaluate_mult() {
        let program: Vec<Token> = vec![Token::Push(3), Token::Push(4), Token::Mul];
        let expected = vec![12];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack, expected);
    }

    #[test]
    fn test_evaluate_div() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Push(5), Token::Div];
        let expected = vec![4];
        let mut vm: VirtualMachine = VirtualMachine::new();
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack, expected);
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Add];
        let mut vm: VirtualMachine = VirtualMachine::new();
        assert_eq!(vm.evaluate(program), Err("Syntax Error"));
    }
}
