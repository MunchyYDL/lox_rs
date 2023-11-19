use crate::scanner::Scanner;
use std::{
    env,
    io::{self, BufRead, Write},
    process,
};

mod scanner;
mod token;

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
