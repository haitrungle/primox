use std::error::Error;
use std::fmt::Display;

use crate::expr::*;
use crate::stmt::Stmt;
use crate::token::*;
use crate::token_type::TokenType::{self, *};
use crate::Lox;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
pub(crate) struct ParseError {
    token: Token,
    message: String,
}

impl Parser {
    pub(crate) fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_owned(),
            current: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let statements = vec![];
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.current_is(&[PRINT]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(&SEMICOLON, "Expect ';' after value.");
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(&SEMICOLON, "Expect ';' after expression.");
        Ok(Stmt::Expression(expr))
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.current_is(&[COMMA]) {
            let comma = self.previous();
            let right = self.ternary()?;
            expr = Binary::new(expr, comma, right).into();
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.current_is(&[QUESTION]) {
            let mid = self.expression()?;
            self.consume(&COLON, "Expect ':' in ternary expression.")?;
            let right = self.ternary()?;
            expr = Ternary::new(expr, mid, right).into();
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.current_is(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator, right).into();
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut term = self.term()?;

        while self.current_is(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term()?;
            term = Binary::new(term, operator, right).into();
        }

        Ok(term)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut factor = self.factor()?;

        while self.current_is(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            factor = Binary::new(factor, operator, right).into();
        }

        Ok(factor)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut unary = self.unary()?;

        while self.current_is(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            unary = Binary::new(unary, operator, right).into();
        }

        Ok(unary)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.current_is(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Unary::new(operator, right).into())
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.current_is(&[FALSE]) {
            Ok(Literal::new(LiteralToken::Bool(false)).into())
        } else if self.current_is(&[TRUE]) {
            Ok(Literal::new(LiteralToken::Bool(true)).into())
        } else if self.current_is(&[NIL]) {
            Ok(Literal::new(LiteralToken::Null).into())
        } else if self.current_is(&[NUMBER, STRING]) {
            Ok(Literal::new(self.previous().literal).into())
        } else if self.current_is(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(&RIGHT_PAREN, "Expect ')' after expression.")?;
            Ok(Grouping::new(expr).into())
        } else {
            Err(ParseError::new(&self.peek(), "Expect expression."))
        }
    }

    fn consume(&mut self, ty: &TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(ty) {
            Ok(self.advance())
        } else {
            Err(ParseError::new(&self.peek(), message))
        }
    }

    fn current_is(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty) {
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

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == SEMICOLON {
                return;
            }

            match self.peek().ty {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}

impl ParseError {
    fn new(token: &Token, message: &str) -> Self {
        Self {
            token: token.clone(),
            message: message.to_string(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.token.ty == EOF {
            write!(
                f,
                "{}",
                Lox::error_message(self.token.line, " at end", &self.message),
            )
        } else {
            write!(
                f,
                "{}",
                Lox::error_message(
                    self.token.line,
                    &format!(" at '{}'", &self.token.lexeme),
                    &self.message
                ),
            )
        }
    }
}

impl Error for ParseError {}
