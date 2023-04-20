#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Add,
}

impl Operator {
    fn from_char(c: &char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OperatorToken(Operator);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ValueToken(i32);

impl ValueToken {
    fn from_string(str: &String) -> Option<Self> {
        match str.parse::<i32>() {
            Ok(n) => Some(Self(n)),
            Err(..) => None,
        }
    }
}

impl std::fmt::Display for ValueToken {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Operator(OperatorToken),
    Value(ValueToken),
}

pub fn evaluate(tokens: &mut Vec<Token>) -> Result<Vec<ValueToken>, &'static str> {
    let mut stack: Vec<ValueToken> = Vec::new();
    while let Some(token) = tokens.pop() {
        match token {
            Token::Operator(_operator) => {
                let left = match stack.pop() {
                    Some(value) => value,
                    None => return Err("Syntax error"),
                };
                let right = match stack.pop() {
                    Some(value) => value,
                    None => return Err("Syntax error"),
                };
                stack.push(ValueToken(left.0 + right.0));
            }
            Token::Value(value) => {
                stack.push(value);
            }
        }
    }
    Ok(stack)
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, &str> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token: String = String::new();
    for char in source.chars() {
        match char {
            '0'..='9' => {
                current_token.push(char);
            }
            _ => {
                if current_token.len() > 0 {
                    match ValueToken::from_string(&current_token) {
                        Some(token) => tokens.push(Token::Value(token)),
                        None => return Err("Syntax error, token not numeric"),
                    }
                    current_token = String::new();
                }
                if let Some(o) = Operator::from_char(&char) {
                    tokens.push(Token::Operator(OperatorToken(o)));
                    continue;
                }
            }
        }
    }
    tokens.reverse();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source = "5 10 +";
        let tokens = tokenize(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(OperatorToken(Operator::Add)),
                Token::Value(ValueToken(10)),
                Token::Value(ValueToken(5)),
            ]
        );
    }

    #[test]
    fn test_tokenize_operator_without_space() {
        let source = "55++";
        let tokens = tokenize(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Operator(OperatorToken(Operator::Add)),
                Token::Operator(OperatorToken(Operator::Add)),
                Token::Value(ValueToken(55)),
            ]
        );
    }

    #[test]
    fn test_evaluate_simple() {
        let source = "5 5 + ";
        let mut tokens = tokenize(source).unwrap();
        let expected = vec![ValueToken(10)];
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
