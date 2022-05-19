use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub(crate) enum LiteralToken {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}

impl LiteralToken {
    pub(crate) fn print(&self) -> String {
        match self {
            LiteralToken::Null => "nil".to_string(),
            LiteralToken::Bool(b) => format!("{}", b),
            LiteralToken::Number(n) => format!("{}", n),
            LiteralToken::String(s) => format!("\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) ty: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: LiteralToken,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: LiteralToken,
        line: usize,
    ) -> Self {
        Self {
            ty: token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}
