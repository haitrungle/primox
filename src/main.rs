use std::env;
use std::process;

use lox_rust::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = Lox::new();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        program.run_file(&args[2]);
    } else {
        program.run_prompt();
    }
}
