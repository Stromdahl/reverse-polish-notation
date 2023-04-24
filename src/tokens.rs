#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Push(i32),

    // Binary operators
    Add,
    Sub,
    Mul,
    Div,
}
