use crate::tokens;

pub struct Lexer<'a> {
    chars: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Self {
        Self { chars }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = tokens::Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.chars.is_empty() {
            // Skip whitespace
            let c = self.chars.chars().next().unwrap();
            if c.is_ascii_whitespace() {
                self.chars = &self.chars[c.len_utf8()..];
                continue;
            }

            // Parse a number
            if c.is_ascii_digit() {
                let end = self
                    .chars
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(self.chars.len());
                let num_str = &self.chars[..end];
                self.chars = &self.chars[end..];
                let num = num_str.parse::<i32>().unwrap();
                return Some(tokens::Token::Push(num));
            }

            // Parse a variable name
            if c.is_ascii_alphabetic() {
                let end = self
                    .chars
                    .find(|c: char| !c.is_ascii_alphanumeric() && c != '\'')
                    .unwrap_or(self.chars.len());
                let var_str = &self.chars[..end];
                self.chars = &self.chars[end..];
                if var_str.ends_with('\'') {
                    return Some(tokens::Token::Store(&var_str[..var_str.len() - 1]));
                } else {
                    return Some(tokens::Token::Load(var_str));
                }
            }

            // Parse an operator
            match c {
                '+' => {
                    self.chars = &self.chars[c.len_utf8()..];
                    return Some(tokens::Token::Add);
                }
                '-' => {
                    self.chars = &self.chars[c.len_utf8()..];
                    return Some(tokens::Token::Sub);
                }
                '*' => {
                    self.chars = &self.chars[c.len_utf8()..];
                    return Some(tokens::Token::Mul);
                }
                '/' => {
                    self.chars = &self.chars[c.len_utf8()..];
                    return Some(tokens::Token::Div);
                }
                '!' => {
                    self.chars = &self.chars[c.len_utf8()..];
                    return Some(tokens::Token::Pop);
                }

                _ => {
                    panic!("Unexpected character {:?}", c);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_pop() {
        let source = "!";
        let mut lexer = Lexer::new(source);
        assert_eq!(
            Some(tokens::Token::Pop),
            lexer.next(),
            "Should have returned Pop"
        );
    }

    #[test]
    fn test_tokenize_store_varable() {
        let source = "bar3'";
        let mut lexer = Lexer::new(source);
        assert_eq!(
            Some(tokens::Token::Store("bar3")),
            lexer.next(),
            "Should have returned Store"
        );
    }

    #[test]
    fn test_tokenize_load_varable() {
        let source = "bar3";
        let mut lexer = Lexer::new(source);
        assert_eq!(
            Some(tokens::Token::Load("bar3")),
            lexer.next(),
            "Should have returned Load"
        );
    }

    #[test]
    fn test_tokenize_number_literals() {
        let source = "55 5";
        let mut lexer = Lexer::new(source);
        assert_eq!(
            Some(tokens::Token::Push(55)),
            lexer.next(),
            "Should have returned number 55"
        );
        assert_eq!(
            Some(tokens::Token::Push(5)),
            lexer.next(),
            "Should have returned number 5"
        );
    }

    #[test]
    fn test_tokenize_operators() {
        let source = "+-*/";
        let mut lexer = Lexer::new(source);
        assert_eq!(
            Some(tokens::Token::Add),
            lexer.next(),
            "Should have returned ADD token"
        );
        assert_eq!(
            Some(tokens::Token::Sub),
            lexer.next(),
            "Should have returned Sub token"
        );
        assert_eq!(
            Some(tokens::Token::Mul),
            lexer.next(),
            "Should have returned Mul token"
        );
        assert_eq!(
            Some(tokens::Token::Div),
            lexer.next(),
            "Should have returned Div token"
        );
        assert_eq!(None, lexer.next(), "Should have exhausted");
    }
}
