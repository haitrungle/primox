use crate::expr::*;
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

    fn expression(&mut self) -> Expr {
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

    fn comparison(&mut self) -> Expr {
        let mut term: Expr = self.term();

        while self.current_is(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            term = Binary::new(term, operator, right).into();
        }

        term
    }

    fn term(&mut self) -> Expr {
        let mut factor: Expr = self.factor();

        while self.current_is(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            factor = Binary::new(factor, operator, right).into();
        }

        factor
    }

    fn factor(&mut self) -> Expr {
        let mut unary: Expr = self.unary();

        while self.current_is(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            unary = Binary::new(unary, operator, right).into();
        }

        unary
    }

    fn unary(&mut self) -> Expr {
        if self.current_is(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            Unary::new(operator, right).into()
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if self.current_is(&[FALSE]) {
            Literal::new(Some(LiteralToken::Bool(false))).into()
        } else if self.current_is(&[TRUE]) {
            Literal::new(Some(LiteralToken::Bool(true))).into()
        } else if self.current_is(&[NIL]) {
            Literal::new(None).into()
        } else if self.current_is(&[NUMBER, STRING]) {
            Literal::new(self.previous().literal).into()
        } else if self.current_is(&[LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Expect ')' after expression.");
            Grouping::new(expr).into()
        } else {
            todo!()
        }
    }

    fn consume(&mut self, ty: TokenType, message: &str) -> Token {
        todo!()
    }

    fn current_is(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty) {
                // Not sure we need to consume the token
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().ty == ty
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
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
