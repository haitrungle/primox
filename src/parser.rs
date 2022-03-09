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

    fn current_is(&mut self, types: &[TokenType]) -> bool {
        for &ty in types {
            if self.check(ty) {
                // Not sure we need to consume the token
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ty == ty
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
