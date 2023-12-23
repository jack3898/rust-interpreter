use crate::types::expr::Expr;

pub enum Stmt {
    Expr { expr: Expr },
    Print { expr: Expr },
}
