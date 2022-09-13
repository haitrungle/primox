use std::env;
use std::process;

use primox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program = Lox::new();
    if args.len() > 2 {
        println!("Usage: primox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        program.run_file(&args[1]);
    } else {
        program.run_prompt();
    }
}
