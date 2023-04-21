#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

impl Operator {
    pub fn from_char(c: &char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mult),
            '/' => Some(Operator::Div),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OperatorToken(pub Operator);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ValueToken(pub i32);

impl ValueToken {
    pub fn from_string(str: &String) -> Option<Self> {
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

