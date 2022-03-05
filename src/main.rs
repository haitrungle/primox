use std::io::Write;
use std::env;
use std::io;
use std::error::Error;
use std::fs;
use std::process;

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

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let content: String = fs::read_to_string(path)?;
        self.run(&content);
        Ok(())
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().expect("flush failed!");

            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else {
                        self.run(&buffer);
                    }
                },
                Err(error) => {
                    println!("error: {error}");
                    break;
                },
            }
        }
    }

    fn run(&mut self, source: &str) {
        print!("{}", source);
    }

    fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn report(line: usize, err: &str, message: &str) {
        println!("[line {line}] Error {err}: {message}");
    }
}