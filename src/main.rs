fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => run_file(args[1].clone()),
        1 => run_prompt(),
        _ => {
            println!("Usage: lox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: String) {
    println!("run_file: {path}");
}

fn run_prompt() {
    println!("run_prompt");
}
