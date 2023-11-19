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

fn run(content: String) {
    println!("run:\n{content}\n");
}
