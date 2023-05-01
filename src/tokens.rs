#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Push(i32),
    Pop,

    Load(&'a str),
    Store(&'a str),

    // Binary operators
    Add,
    Sub,
    Mul,
    Div,
}


