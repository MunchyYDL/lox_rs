use std::{
    env,
    io::{self, BufRead, Write},
    process,
};

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
    let file_content = std::fs::read_to_string(path).unwrap(); // No error handling needed... 😛
    run(file_content);
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

#[derive(Debug)]
struct Token {}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner { source }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}
