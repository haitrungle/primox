use crate::token::{LiteralToken, Token};
use derive_new::new;

#[derive(Debug)]
pub(crate) enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Ternary(Box<Ternary>),
    Unary(Box<Unary>),
}

#[derive(new, Debug)]
pub(crate) struct Binary {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

#[derive(new, Debug)]
pub(crate) struct Grouping {
    pub(crate) expression: Expr,
}

#[derive(new, Debug)]
pub(crate) struct Literal {
    pub(crate) value: Option<LiteralToken>,
}

#[derive(new, Debug)]
pub(crate) struct Ternary {
    pub(crate) left: Expr,
    pub(crate) mid: Expr,
    pub(crate) right: Expr,
}

#[derive(new, Debug)]
pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl From<Binary> for Expr {
    fn from(b: Binary) -> Self {
        Self::Binary(Box::new(b))
    }
}

impl From<Grouping> for Expr {
    fn from(b: Grouping) -> Self {
        Self::Grouping(Box::new(b))
    }
}

impl From<Literal> for Expr {
    fn from(b: Literal) -> Self {
        Self::Literal(Box::new(b))
    }
}

impl From<Ternary> for Expr {
    fn from(b: Ternary) -> Self {
        Self::Ternary(Box::new(b))
    }
}

impl From<Unary> for Expr {
    fn from(b: Unary) -> Self {
        Self::Unary(Box::new(b))
    }
}
