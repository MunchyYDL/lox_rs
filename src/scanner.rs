#![allow(dead_code)]

use lazy_static::*;
use std::collections::HashMap;

use crate::token::{Token, TokenType};

static ZERO_TERMINATED: char = '\0';

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While)
    ]);
}

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
        let token = Token::new(TokenType::Eof, "".into(), None, self.line);
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
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

            // Number
            c if c.is_ascii_digit() => self.number(),

            // Identifier or Keyword - Have to start with character or _
            c if self.is_alpha(c) => self.identifier(),

            // Default - If we reach this, we have gotten something unexpected
            c => {
                crate::error(
                    self.line,
                    format!("Unexpected character code: '{:#x}'.", c as u32),
                );
            }
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }

    fn identifier(&mut self) {
        let mut d = self.peek();
        while self.is_alpha_numeric(d) {
            self.advance();
            d = self.peek();
        }

        let text = self.get_text();
        let token_type = KEYWORDS
            .get(text.as_str())
            .unwrap_or(&TokenType::Identifier);
        self.add_token(token_type.clone());
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the .
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Read the value from the source & convert it
        let text = self.get_text();
        let number = text.parse::<f32>().unwrap();

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

        // Get the text & trim the surrounding quotes
        let text = &self.get_text();
        let text = &text[1..text.len() - 1];

        dbg!(text);

        self.add_token_object(TokenType::String, Some(text.into()));
    }

    fn advance(&mut self) -> char {
        let c = self.get_char();
        self.current += 1;
        c
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.get_char() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return ZERO_TERMINATED;
        }
        self.get_char()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return ZERO_TERMINATED;
        }
        self.get_next_char()
    }

    // Return the char from self.source at self.current
    fn get_char(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    // Return the char from self.source at self.current + 1
    fn get_next_char(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap()
    }

    /// Returns the String from self.source between self.start and self.current
    fn get_text(&self) -> String {
        self.source[self.start..self.current].into()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None)
    }

    fn add_token_object(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.get_text();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }
}
