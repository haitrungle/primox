mod ast_printer;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

#[cfg(feature = "rpn-printer")]
mod rpn_printer;

use std::fs;
use std::io;
use std::io::Write;
use std::process;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub struct Lox {
    interpreter: Interpreter,
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        let interpreter = Interpreter::new();
        Self { interpreter, had_error: false, had_runtime_error: false, }
    }

    pub fn run_file(&mut self, path: &str) {
        let content: String = fs::read_to_string(path).unwrap();
        self.run(content);
        if self.had_error {
            process::exit(65);
        }
        if self.had_runtime_error {
            process::exit(70);
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
        let tokens = scanner.scan_tokens(self);

        let mut parser = Parser::new(&tokens);
        let statements = match parser.parse() {
            Ok(expr) => expr,
            Err(e) => {
                self.error(e);
                return;
            }
        };

        self.interpreter.interprete(statements);
    }

    fn error_message(line: usize, err: &str, message: &str) -> String {
        format!("[line {line}] Error{err}: {message}")
    }

    fn error(&mut self, e: impl std::error::Error) {
        self.had_error = true;
        println!("{}", e);
    }

    fn runtime_error(&mut self, e: impl std::error::Error) {
        self.had_runtime_error = true;
        println!("{}", e);
    }
}
