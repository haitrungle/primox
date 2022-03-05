use crate::token_type::TokenType;

#[derive(Debug)]
struct Token<T> {
  _type: TokenType,
  lexeme: String,
  literal: T,
  line: usize,
}

impl<T> Token<T> {
  fn new(token_type: TokenType, lexeme: String, literal: T, line: usize) -> Self {
    Self {
      _type: token_type,
      lexeme,
      literal,
      line,
    }
  }
}