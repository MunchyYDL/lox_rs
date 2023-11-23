#![allow(dead_code)]

use crate::token::{Token, TokenType, Literal};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static ZERO_TERMINATED: char = '\0';

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    HashMap::from([
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
        ("while", TokenType::While),
    ])
});

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    /*
     * Public methods - Creating a Scanner and staring a scan for tokens
     */

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
        self.add_token(TokenType::Eof);

        // Return the list of tokens
        self.tokens.clone()
    }

    /*
     * Private methods - All the implementation details of how the scanning is done
     */

    /// This method looks at the next char in the source, and makes decisions based on that
    fn scan_token(&mut self) {
        let c = self.peek();
        self.advance();

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
            '\n' => self.next_line(),

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

    /*
     * The large classification scanners: identifier, number and string
     */

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
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

        self.add_token_literal(TokenType::Number, Some(Literal::Number(number)));
    }

    fn string(&mut self) {
        // Try to find the terminating " of the string
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.next_line();
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

        self.add_token_literal(TokenType::String, Some(Literal::String(text.into())));
    }

    /*
     * Small helpers for the scanning
     */

    /// Original name: advance
    fn advance(&mut self) {
        self.current += 1;
    }

    fn next_line(&mut self) {
        self.line += 1;
    }

    fn next_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    /*
     * Small helpers to classify if the current char is alpha or alpha-numeric
     */

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /*
     * Checking that we don't pass the end of the source
     */

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /*
     * Peeking at characters in the source
     */

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

    /*
     * Getting chars or text from the source
     */

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

    /*
     * Adding tokens
     */

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.get_text();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(token);
    }
}
