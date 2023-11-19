// 4.2 - Lexemes and Tokens
#![allow(dead_code)]

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    #[allow(clippy::inherent_to_string)]
    fn to_string(&self) -> String {
        format!("{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

// FIXME: Non Rusty code...

// https://doc.rust-lang.org/rustc/?search=non_camel_case_types
// https://rust-lang.github.io/rust-clippy/master/index.html#/upper_case_acronyms
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug)]
enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
