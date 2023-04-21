use crate::tokens;

pub fn perform_operation(
    operator: tokens::Operator,
    stack: &mut Vec<tokens::ValueToken>,
) -> Result<(), &'static str> {
    let left = match stack.pop() {
        Some(value) => value.0,
        None => return Err("Syntax error"),
    };
    let right = match stack.pop() {
        Some(value) => value.0,
        None => return Err("Syntax error"),
    };

    let result = match operator {
        tokens::Operator::Add => right + left,
        tokens::Operator::Sub => right - left,
        tokens::Operator::Mult => right * left,
        tokens::Operator::Div => right / left,
    };
    stack.push(tokens::ValueToken(result));
    Ok(())
}

pub fn evaluate(tokens: &mut Vec<tokens::Token>) -> Result<Vec<tokens::ValueToken>, &'static str> {
    let mut stack: Vec<tokens::ValueToken> = Vec::new();
    while let Some(token) = tokens.pop() {
        match token {
            tokens::Token::Operator(operator) => {
                perform_operation(operator.0, &mut stack)?;
            }
            tokens::Token::Value(value) => {
                stack.push(value);
            }
        }
    }
    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;
    #[test]
    fn test_evaluate_add() {
        let source = "5 5 +";
        let mut tokens = tokenize(source).unwrap();
        let expected = vec![tokens::ValueToken(10)];
        assert_eq!(evaluate(&mut tokens), Ok(expected));
    }

    #[test]
    fn test_evaluate_sub() {
        let source = "3 4 -";
        let mut tokens = tokenize(source).unwrap();
        let expected = vec![tokens::ValueToken(-1)];
        assert_eq!(evaluate(&mut tokens), Ok(expected));
    }

    #[test]
    fn test_evaluate_mult() {
        let source = "3 4 *";
        let mut tokens = tokenize(source).unwrap();
        let expected = vec![tokens::ValueToken(12)];
        assert_eq!(evaluate(&mut tokens), Ok(expected));
    }

    #[test]
    fn test_evaluate_div() {
        let source = "20 5 /";
        let mut tokens = tokenize(source).unwrap();
        let expected = vec![tokens::ValueToken(4)];
        assert_eq!(evaluate(&mut tokens), Ok(expected));
    }

    #[test]
    fn test_evaluate_should_return_syntax_error() {
        let source = "5 5 ++";
        let mut tokens = tokenize(source).unwrap();
        let result = evaluate(&mut tokens);
        assert_eq!(result, Err("Syntax error"));
    }
}
