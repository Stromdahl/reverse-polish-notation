
#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
        struct OperatorToken {
    value: Operator,
}

#[derive(Debug, PartialEq)]
struct ValueToken {
    value: i32,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(OperatorToken),
    Value(ValueToken),
}

#[derive(Debug)]
struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn new(tokens: Vec<Token>) -> Expression {
        Expression { tokens }
    }

    fn evaluate(&self) -> Vec<i32>{
        let mut stack = Vec::new();
        for token in &self.tokens {
            match token {
                Token::Value(value) => stack.push(value.value),
                Token::Operator(operator) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let result = match operator.value {
                        Operator::Add => left + right,
                        Operator::Subtract => left - right,
                        Operator::Multiply => left * right,
                        Operator::Divide => left / right,
                    };
                    stack.push(result);
                }
            }
        }
        stack
    }
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for token in source.split_whitespace() {
        if let Ok(value) = token.parse::<i32>() {
            tokens.push(Token::Value(ValueToken { value }));
        } else if let Some(operator) = Operator::from_str(token) {
            tokens.push(Token::Operator(OperatorToken { value: operator }));
        }
    }
    tokens
}

fn main() {
    let source = "5 5 5 + +";
    let tokens = tokenize(source);
    let expression = Expression::new(tokens);
    println!("{:?}", expression.evaluate());
}

// Reverse Polish Notation Example
// 5 10 +
// 15

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source = "5 10 +";
        let tokens = tokenize(source);
        assert_eq!(
            tokens,
            vec![
                Token::Value(ValueToken { value: 5 }),
                Token::Value(ValueToken { value: 10 }),
                Token::Operator(OperatorToken {
                    value: Operator::Add
                }),
            ]
        );
    }
}
