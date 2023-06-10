mod error;
mod scanner;
mod token;
mod token_type;

use std::io::Write;

use scanner::Scanner;

pub fn run(args: Vec<String>) {
    if args.len() > 2 {
        println!("Usage: rslox [script]");
        std::process::exit(64);
    }

    if args.len() == 2 {
        run_file(args[0].clone());
    } else {
        run_prompt();
    }
}

fn run_file(path: String) {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(1);
        }
    };

    run_source(source);

    if error::had_error() {
        std::process::exit(65);
    }
}

fn run_prompt() {
    loop {
        print!("> ");

        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to flush stdout: {}", err);
                std::process::exit(1);
            }
        }

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to read line: {}", err);
                std::process::exit(1);
            }
        }

        run_source(line);

        error::reset_error();
    }
}

fn run_source(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}