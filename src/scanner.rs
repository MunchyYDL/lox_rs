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
        let token = Token::new(TokenType::Eof, "".into(), "".into(), 1);
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        todo!()
    }
}
