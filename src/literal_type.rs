#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lit::String(s) => write!(f, "{}", s),
            Lit::Number(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Nil => write!(f, "nil"),
        }
    }
}
