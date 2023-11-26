#![allow(dead_code)]

use std::fmt::{self, Display};
use strum::Display;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::String | TokenType::Number => write!(
                f,
                "{:<10} {:?}",
                self.token_type,
                self.literal.as_ref().unwrap()
            ),
            TokenType::Eof => write!(f, "{:<10}", self.token_type),
            _ => write!(f, "{:<10} '{}'", self.token_type, self.lexeme),
        }
    }
}

#[derive(Display, Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
