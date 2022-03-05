use std::io::Write;
use std::env;
use std::io;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[2]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let content: String = fs::read_to_string(path)?;
    run(&content);
    Ok(())
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().expect("flush failed!");

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                } else {
                    run(&buffer);
                }
            },
            Err(error) => {
                println!("error: {}", error);
                break;
            },
        }
    }
}

fn run(source: &str) {
    print!("{}", source);
}