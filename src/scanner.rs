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
            // Single-character tokens.
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

            // One or two character tokens.
            '!' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.next_matches('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            // Comment or single slash
            '/' => {
                if self.next_matches('/') {
                    // A comment goes until the end of the line, but brings no
                    // value to the evaluation of the program, so it doesn't
                    // add anything to the list of found/scanned tokens.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // Ignore whitespace
            ' ' | '\r' | '\t' => {}

            // Line breaks
            '\n' => self.line += 1,

            // String
            '"' => self.string(),

            // Default
            x => {
                if self.is_digit(x) {
                    self.number();
                }

                // If we reach this, we have gotten something unexpected
                crate::error(
                    self.line,
                    format!("Unexpected character: '{:#x}'.", x as u32),
                );
            }
        }
    }

    fn is_digit(&mut self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) {
        let mut d = self.peek();
        while self.is_digit(d) {
            self.advance();
            d = self.peek();
        }

        // Look for fractional part
        d = self.peek_next();
        if self.peek() == '.' && self.is_digit(d) {
            // Consume the .
            self.advance();

            d = self.peek();
            while self.is_digit(d) {
                self.advance();
                d = self.peek();
            }
        }

        // Read the value from the source & convert it
        let value = &self.source[self.start..self.current];
        let number = value.parse::<f32>().unwrap();

        // Our current implementation only allows Option<String> to be passed here,
        // so I'll convert it back to a string to let it pass for now.

        // FIXME: Add support for both string and number literals!
        self.add_token_object(TokenType::Number, Some(number.to_string()));
    }

    fn string(&mut self) {
        // Try to find the terminating " of the string
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, "Unterminated string.".into());
            return;
        }

        // Consume the closing "
        self.advance();

        // Trim the surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_object(TokenType::String, Some(value.into()));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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
