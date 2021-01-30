use std::env;
use std::io;
use std::io::Write;

#[macro_use]
extern crate lazy_static;

mod rlox;
pub mod scanner;
pub mod tokens;

fn args_valid() -> Result<Option<String>, String> {
    let mut args: Vec<String> = env::args()
        .skip(1) //skip executable name
        .collect();

    return if args.len() == 1 {
        Ok(args.pop())
    } else if args.is_empty() {
        Ok(None)
    } else {
        Err("Invalid".to_string())
    };
}

fn run_prompt() {
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        rlox::run(&buffer);
        buffer.clear();
    }
}

fn run_file(path: &String) {
    println!("on path {}", path);
}

fn main() {
    let had_error = false;

    match args_valid() {
        Ok(Some(ref s)) => {
            run_file(s);
        }
        Ok(None) => {
            run_prompt();
        }
        Err(_e) => {
            println!("Usage: rlox [script]");
            std::process::exit(1);
        }
    };
}
