use crate::expr::{Binary, Expr};
use crate::token::*;
use crate::token_type::TokenType::{self, *};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_owned(),
            current: 0,
        }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.current_is(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary::new(expr, operator, right).into();
        }

        expr
    }
}
