#![allow(dead_code)]

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                break;
            };
            self.start = self.current;
            self.scan_token();
        }

        // End with a Eof token
        let token = Token::new(TokenType::Eof, "".into(), None, 1);
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
      let c = self.advance();
      match c {
        '(' => self.add_token(TokenType::LeftParen),
        ')' => self.add_token(TokenType::RightParen),
        '{' => self.add_token(TokenType::LeftBrace),
        '}' => self.add_token(TokenType::RightBrace),
        ',' => self.add_token(TokenType::Comma),
        '.' => self.add_token(TokenType::Dot),
        '-' => self.add_token(TokenType::Minus),
        '+' => self.add_token(TokenType::Plus),
        ';' => self.add_token(TokenType::Semicolon),
        '*' => self.add_token(TokenType::Star),
        _ => eprintln!("Missing token: {c}")
      }
    }

    fn advance(&mut self) -> char {
      let c = self.source.chars().nth(self.current);
      self.current += 1;
      c.unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
      self.add_token_object(token_type, None)
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<String>) {
      let text = &self.source[self.start..self.current];
      let token = Token::new(token_type, text.into(), literal, self.line);
      self.tokens.push(token);
    }
  }