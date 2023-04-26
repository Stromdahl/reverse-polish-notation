use crate::tokens::Token;
pub struct VirtualMachine {
    pub stack: Vec<i32>,
}

fn try_pop_operators(stack: &mut Vec<i32>) -> Result<(i32, i32), &'static str> {
    Ok((
        stack.pop().ok_or("Syntax Error")?,
        stack.pop().ok_or("Syntax Error")?,
    ))
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn evaluate(&mut self, program: Vec<Token>) -> Result<(), &'static str> {
        for token in program {
            match token {
                Token::Push(x) => self.stack.push(x),
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
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
        let mut vm: VirtualMachine = VirtualMachine { stack: Vec::new() };
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack, expected);
    }

    #[test]
    fn test_evaluate_div() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Push(5), Token::Div];
        let expected = vec![4];
        let mut vm: VirtualMachine = VirtualMachine { stack: Vec::new() };
        vm.evaluate(program).unwrap();
        assert_eq!(vm.stack, expected);
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Add];
        let mut vm: VirtualMachine = VirtualMachine { stack: Vec::new() };
        assert_eq!(vm.evaluate(program), Err("Syntax Error"));
    }
}
