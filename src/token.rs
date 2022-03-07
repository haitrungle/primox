use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub(crate) enum LiteralToken {
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    _type: TokenType,
    pub(crate) lexeme: String,
    literal: Option<LiteralToken>,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<LiteralToken>,
        line: usize,
    ) -> Self {
        Self {
            _type: token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}
