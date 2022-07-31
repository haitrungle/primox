use crate::expr::Expr;
use derive_new::new;

pub(crate) enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}

#[derive(new, Debug)]
pub(crate) struct ExpressionStmt {
    pub(crate) expression: Expr,
}

#[derive(new, Debug)]
pub(crate) struct PrintStmt {
    pub(crate) expression: Expr,
}
