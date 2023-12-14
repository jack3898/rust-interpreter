#[derive(Debug)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
