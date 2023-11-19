fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("Usage: lox [script]");
    } else if args.len() == 1 {
        run_file(args[0].clone());
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    todo!()
}

fn run_file(_args: String) {
    todo!()
}

