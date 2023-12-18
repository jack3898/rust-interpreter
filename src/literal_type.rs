#[derive(Debug, PartialEq)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => write!(f, "{}", n),
            LiteralType::Boolean(b) => write!(f, "{}", b),
            LiteralType::Nil => write!(f, "nil"),
        }
    }
}
