use crate::tokens;

pub fn tokenize(source: &str) -> Result<Vec<tokens::Token>, &'static str> {
    let mut program: Vec<tokens::Token> = Vec::new();
    let mut current_token = String::new();
    for char in source.chars() {
        match char {
            '0'..='9' => current_token.push(char),
            _ => {
                if current_token.len() > 0 {
                    let token = match current_token.parse::<i32>() {
                        Ok(value) => tokens::Token::Push(value),
                        Err(..) => return Err("Syntax Error: Not a number"),
                    };
                    current_token = String::new();
                    program.push(token);
                }

                match char {
                    '+' => program.push(tokens::Token::Add),
                    '-' => program.push(tokens::Token::Sub),
                    '*' => program.push(tokens::Token::Mul),
                    '/' => program.push(tokens::Token::Div),
                    _ => {}
                }

            }
        }
    }
    if current_token.len() > 0 {
        let token = match current_token.parse::<i32>() {
            Ok(value) => tokens::Token::Push(value),
            Err(..) => return Err("Syntax Error: Not a number"),
        };
        program.push(token);
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

    #[test]
    fn test_tokenize_number() {
        let source = "64";
        let program = tokenize(source).unwrap();
        assert_eq!(program, vec![tokens::Token::Push(64),]);
    }

    #[test]
    fn test_tokenize_expression() {
        let source = "5 55+";
        let program = tokenize(source).unwrap();
        assert_eq!(program, vec![
            tokens::Token::Push(5),
            tokens::Token::Push(55),
            tokens::Token::Add,
        ]);
    }
}


