use crate::tokens;
pub fn tokenize(source: &str) -> Result<Vec<tokens::Token>, &'static str> {
    let mut program: Vec<tokens::Token> = Vec::new();
    let mut current_token = String::new();
    for char in source.chars() {
        match char {
            '0'..='9' => current_token.push(char),
            _ => {
                match char {
                    '+' => program.push(tokens::Token::Add),
                    '-' => program.push(tokens::Token::Sub),
                    '*' => program.push(tokens::Token::Mul),
                    '/' => program.push(tokens::Token::Div),
                    _ => {},
                }
            }
        }
    }
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize_operators() {
        let source = "+-*/";
        let program = tokenize(source).unwrap();
        assert_eq!(
            program,
            vec![
                tokens::Token::Add,
                tokens::Token::Sub,
                tokens::Token::Mul,
                tokens::Token::Div,
            ]
        );
    }
}

