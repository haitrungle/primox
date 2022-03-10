mod ast_printer;
mod expr;
mod parser;
mod rpn_printer;
mod scanner;
mod token;
mod token_type;

use std::fs;
use std::io;
use std::io::Write;
use std::process;

use ast_printer::AstPrinter;
use parser::Parser;
use scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        let content: String = fs::read_to_string(path).unwrap();
        self.run(content);
        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            // https://stackoverflow.com/a/34993992
            io::stdout().flush().expect("flush failed!");

            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    } else {
                        self.run(buffer);
                        self.had_error = false;
                    }
                }
                Err(error) => {
                    println!("error: {error}");
                    break;
                }
            }
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(&tokens);
        let expression = parser.parse();

        // Stop if there was a syntax error.
        if self.had_error {
            return;
        }

        println!("{}", AstPrinter::print(expression));
    }

    fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn error_message(line: usize, err: &str, message: &str) -> String {
        format!("[line {line}] Error{err}: {message}")
    }

    fn report(line: usize, err: &str, message: &str) {
        println!("{}", Self::error_message(line, err, message));
    }
}
