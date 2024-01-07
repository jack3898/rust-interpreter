use crate::types::{expr::Expr, token::Tok};

pub enum Stmt {
    Expr { expr: Expr },
    Print { expr: Expr },
    Var { name: Tok, expr: Expr },
}
