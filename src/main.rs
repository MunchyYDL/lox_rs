use std::{
    env,
    io::{self, BufRead, Write},
    process,
};

// 4.1 - The Interpreter Framework
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run_file(args[1].clone()),
        1 => run_prompt(),
        _ => {
            println!("Usage: lox [script]");
            process::exit(64);
        }
    }
}

fn run_file(path: String) {
    let file_content = std::fs::read_to_string(path).unwrap();
    run(file_content);

    // Indicate an error in the exit code.
    // if (hadError) System.exit(65);
}

fn run_prompt() {
    println!("Welcome to the Lox REPL");
    println!("-----------------------");
    println!("Ctrl-D to exit\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let line = stdin.lock().lines().next();
        if line.is_none() {
            println!("\n\nBye!");
            break;
        }

        run(line.unwrap().unwrap());
        //hadError = false;
    }
}

fn run(source: String) {
    println!("run:\n{source}\n");

    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // For now, just print the tokens.
    for token in tokens {
        println!("{token:?}");
    }
}

#[allow(dead_code)]
struct Scanner {
    source: String,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner { source }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}

/// 4.1.1 - Error handling
#[allow(dead_code)]
fn error(line: usize, message: String) {
    report(line, "".into(), message);
}

fn report(line: usize, r#where: String, message: String) {
    eprintln!("[line {line}] Error {where}: {message}");

    // I'll come back to this when I can see how it's
    // supposed to be used, as this is not Rusty at all. :(
    // hadError = true;
}

// 4.2 - Lexemes and Tokens
#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

#[allow(dead_code)]
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
#[allow(non_camel_case_types)]
// https://rust-lang.github.io/rust-clippy/master/index.html#/upper_case_acronyms
#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
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
