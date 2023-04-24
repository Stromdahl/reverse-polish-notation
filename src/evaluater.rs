use crate::tokens::Token;

fn try_pop_operators(stack: &mut Vec<i32>) -> Result<(i32, i32), &'static str> {
    Ok((
        stack.pop().ok_or("Syntax Error")?,
        stack.pop().ok_or("Syntax Error")?,
    ))
}

pub fn evaluate(program: Vec<Token>) -> Result<Vec<i32>, &'static str> {
    let mut stack: Vec<i32> = Vec::new();
    for token in program {
        match token {
            Token::Push(x) => stack.push(x),
            Token::Add => {
                let (right, left) = try_pop_operators(&mut stack)?;
                stack.push(left + right);
            }
            Token::Sub => {
                let (right, left) = try_pop_operators(&mut stack)?;
                stack.push(left - right);
            }
            Token::Mul => {
                let (right, left) = try_pop_operators(&mut stack)?;
                stack.push(left * right);
            }
            Token::Div => {
                let (right, left) = try_pop_operators(&mut stack)?;
                stack.push(left / right);
            }
        }
    }
    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let x = 4;
        let y = 3;
        let program: Vec<Token> = vec![Token::Push(x), Token::Push(y), Token::Add];
        assert_eq!(vec![x + y], evaluate(program).unwrap());
    }

    #[test]
    fn test_evaluate_sub() {
        let program: Vec<Token> = vec![Token::Push(3), Token::Push(4), Token::Sub];
        let expected = vec![-1];
        assert_eq!(evaluate(program), Ok(expected));
    }

    #[test]
    fn test_evaluate_mult() {
        let program: Vec<Token> = vec![Token::Push(3), Token::Push(4), Token::Mul];
        let expected = vec![12];
        assert_eq!(evaluate(program), Ok(expected));
    }

    #[test]
    fn test_evaluate_div() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Push(5), Token::Div];
        let expected = vec![4];
        assert_eq!(evaluate(program), Ok(expected));
    }

    #[test]
    fn test_evaluate_syntax_error() {
        let program: Vec<Token> = vec![Token::Push(20), Token::Add];
        assert_eq!(evaluate(program), Err("Syntax Error"));
    }
}
