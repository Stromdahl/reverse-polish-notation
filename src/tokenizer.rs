use crate::tokens;

pub fn tokenize(source: &str) -> Result<Vec<tokens::Token>, &str> {
    let mut tokens: Vec<tokens::Token> = Vec::new();
    let mut current_token: String = String::new();
    for char in source.chars() {
        match char {
            '0'..='9' => {
                current_token.push(char);
            }
            _ => {
                if current_token.len() > 0 {
                    match tokens::ValueToken::from_string(&current_token) {
                        Some(token) => tokens.push(tokens::Token::Value(token)),
                        None => return Err("Syntax error, token not numeric"),
                    }
                    current_token = String::new();
                }
                if let Some(o) = tokens::Operator::from_char(&char) {
                    tokens.push(tokens::Token::Operator(tokens::OperatorToken(o)));
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
    fn test_tokenize_operators() {
        let source = "+-*/";
        let tokens = tokenize(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Div)),
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Mult)),
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Sub)),
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Add)),
            ]
        );
    }

    #[test]
    fn test_tokenize() {
        let source = "5 10 +";
        let tokens = tokenize(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Add)),
                tokens::Token::Value(tokens::ValueToken(10)),
                tokens::Token::Value(tokens::ValueToken(5)),
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
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Add)),
                tokens::Token::Operator(tokens::OperatorToken(tokens::Operator::Add)),
                tokens::Token::Value(   tokens::ValueToken(55)),
            ]
        );
    }

}
