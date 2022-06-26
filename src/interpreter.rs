use std::error::Error;
use std::fmt::Display;

use crate::expr::*;
use crate::token::*;
use crate::token_type::TokenType::*;

// In Java, all generic values of Lox is fitted into Object, using 
// `istanceOf` for type-checking and finding runtime errors. This is
// not feasible in Rust, so for now we just wrap every possible Lox
// values in an enum instead of a Trait. In effect, we have implemented
// a type system for Lox in Rust.
#[derive(PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String)
}

impl From<LiteralToken> for Value {
    fn from(token: LiteralToken) -> Self {
        match token {
            LiteralToken::Null => Value::Null,
            LiteralToken::Bool(b) => Value::Bool(b),
            LiteralToken::Number(n) => Value::Number(n),
            LiteralToken::String(s) => Value::String(s),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "nil"),
            Value::Bool(v) => write!(f, "{}", v),
            Value::Number(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}

pub(crate) struct Interpreter {}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn interprete(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        Self::evaluate(expr)
    }

    fn evaluate(expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Binary(e) => Self::binary_expr(*e),
            Expr::Literal(e) => Self::literal_expr(*e),
            Expr::Grouping(e) => Self::grouping_expr(*e),
            Expr::Ternary(e) => Self::ternary_expr(*e),
            Expr::Unary(e) => Self::unary_expr(*e),
        }
    }

    fn binary_expr(expr: Binary) -> Result<Value, RuntimeError> {
        let left = Self::evaluate(expr.left)?;
        let right = Self::evaluate(expr.right)?;

        match expr.operator.ty {
            GREATER | GREATER_EQUAL | LESS | LESS_EQUAL | MINUS | SLASH | STAR => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        match expr.operator.ty {
                            GREATER => Ok(Value::Bool(a > b)),
                            GREATER_EQUAL => Ok(Value::Bool(a >= b)),
                            LESS => Ok(Value::Bool(a < b)),
                            LESS_EQUAL => Ok(Value::Bool(a <= b)),
                            MINUS => Ok(Value::Number(a - b)),
                            SLASH => Ok(Value::Number(a / b)),
                            STAR => Ok(Value::Number(a * b)),
                            _ => unreachable!(),
                        }
                    }
                    _ => Err(RuntimeError::new(&expr.operator, "Operands must be two numbers.")),
                }
            }
            BANG_EQUAL => Ok(Value::Bool(!Self::is_equal(left, right))),
            EQUAL_EQUAL => Ok(Value::Bool(Self::is_equal(left, right))),
            PLUS => {
                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a+b)),
                    (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                    _ => Err(RuntimeError::new(&expr.operator, "Operands must be two numbers or two strings.")),
                }
            }
            _ => unreachable!(),
        }
    }

    fn literal_expr(expr: Literal) -> Result<Value, RuntimeError> {
        return Ok(expr.value.into());
    }

    fn grouping_expr(expr: Grouping) -> Result<Value, RuntimeError> {
        return Self::evaluate(expr.expression);
    }

    fn ternary_expr(expr: Ternary) -> Result<Value, RuntimeError> {
        let left = Self::evaluate(expr.left)?;
        let mid = Self::evaluate(expr.mid)?;
        let right = Self::evaluate(expr.right)?;

        if Self::is_truthy(left) { Ok(mid) } else { Ok(right) }
    }

    fn unary_expr(expr: Unary) -> Result<Value, RuntimeError> {
        let right = Self::evaluate(expr.right)?;

        match expr.operator.ty {
            BANG => Ok(Value::Bool(!Self::is_truthy(right))),
            MINUS => {
                match right {
                    Value::Number(a) => Ok(Value::Number(-a)),
                    _ => Err(RuntimeError::new(&expr.operator, "Operand must be a number.")),
                }
            }
            _ => unreachable!()
        }
    }

    fn is_truthy(val: Value) -> bool {
        match val {
            Value::Null => false,
            Value::Bool(b) => b,
            _ => true,
        }
    }

    fn is_equal(a: Value, b: Value) -> bool {
        a == b
    }
}

#[derive(Debug)]
pub(crate) struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        Self {
            token: token.clone(),
            message: message.to_string(),
        }
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", &self.message, self.token.line)
    }
}

impl Error for RuntimeError {}
