use crate::token_type::TokenType::{self, *};
use crate::{token::*, Lox};

struct Scanner {
  source: String,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {
  fn new(source: String) -> Self {
    Self {
      source,
      tokens: vec![],
      start: 0,
      current: 0,
      line: 1
    }
  }

  fn scan_tokens(&mut self) -> Vec<Token> {
    while !(self.is_at_end()) {
      self.start = self.current;
      self.scan_token();
    }
  
    let token = Token::new(EOF, "", None, self.line);
    self.tokens.push(token);
    self.tokens.clone()
  }

  fn scan_token(&mut self) {
    let c: char = self.advance();
    match c {
      '(' => self.add_token(LEFT_PAREN, None),
      ')' => self.add_token(RIGHT_PAREN, None),
      '{' => self.add_token(LEFT_BRACE, None),
      '}' => self.add_token(RIGHT_BRACE, None),
      ',' => self.add_token(COMMA, None),
      '.' => self.add_token(DOT, None),
      '-' => self.add_token(MINUS, None),
      '+' => self.add_token(PLUS, None),
      ';' => self.add_token(SEMICOLON, None),
      '*' => self.add_token(STAR, None),

      '!' => {
        let t = if self.next_is('=') { BANG_EQUAL } else { BANG };
        self.add_token(t, None);
      },
      '=' => {
        let t = if self.next_is('=') { EQUAL_EQUAL } else { EQUAL };
        self.add_token(t, None);
      },
      '<' => {
        let t = if self.next_is('=') { LESS_EQUAL } else { LESS };
        self.add_token(t, None);
      },
      '>' => {
        let t = if self.next_is('=') { GREATER_EQUAL } else { GREATER };
        self.add_token(t, None);
      },

      '/' => {
        if self.next_is('/') {
          // A comment goes until the end of the line.
          while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
          }
        } else {
          self.add_token(SLASH, None);
        }
      },

      ' ' | '\r' | '\t' => {},

      '\n' => self.line += 1,

      // TODO: coalesce a run of invalid characters into a single error
      _ => Lox::error(self.line, "Unexpected character"),
    }
  }

  fn next_is(&mut self, expected: char) -> bool {
    if self.is_at_end() {
      false
    } else if self.source.chars().nth(self.current).unwrap() != expected {
      false
    } else {
      self.current += 1;
      true
    }
  }

  fn peek(&self) -> char {
    if self.is_at_end() {
      '\0'
    } else {
      self.source.chars().nth(self.current).unwrap()
    }
  }

  fn advance(&mut self) -> char {
    // TODO: this is wildly inefficient, as it reiterate over the source
    // string everytime. Maybe use the iterator directly?
    self.current += 1;
    self.source.chars().nth(self.current-1).unwrap()
  }

  fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
    let text = self.source.get(self.start..self.current).unwrap();
    let token = Token::new(token_type, text, literal, self.line);
    self.tokens.push(token);
  }

  fn is_at_end(&self) -> bool {
    self.current > self.source.len()
  }
}