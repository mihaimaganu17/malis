pub mod ast;
mod environment;
mod error;
pub mod interpreter;
pub mod resolver;
mod parser;
mod scanner;
mod token;
mod visit;

pub use error::MalisError;
pub use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};
use visit::AstPrinter;

#[derive(Default)]
pub struct Malis {
    // Keeps track of wheather the code has an error and to avoid executing it.
    _had_error: bool,
    interpreter: Interpreter,
}

impl Malis {
    pub fn execute<P: AsRef<Path>>(path: P) -> Result<(), MalisError> {
        let mut malis = Self::default();
        let source = fs::read_to_string(path)?;
        malis.run(source.as_str(), false)
    }

    pub fn run(&mut self, bytes: &str, is_repl: bool) -> Result<(), MalisError> {
        let mut scanner = Scanner::new(bytes);
        let maybe_tokens = scanner.scan_tokens();

        match maybe_tokens {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let stmts = parser.parse()?;
                let mut ast_printer = AstPrinter;

                let ast = if !stmts.is_empty() || !is_repl {
                    let ast = ast_printer.print_stmt(&stmts);
                    self.interpreter.interpret(stmts.as_slice())?;
                    ast
                } else {
                    // Reset the parser such that we could parse in expression form
                    parser.reset();
                    let expr = parser.separator()?;
                    println!("{}", self.interpreter.evaluate(&expr)?);
                    ast_printer.print_expr(&expr)
                };

                println!("Ast {}", ast);
            }
            // Print all the errors we found during scanning
            Err(scanner_errors) => scanner_errors.iter().for_each(|e| println!("{e:?}")),
        }
        Ok(())
    }

    /// Fires up an interactive command prompt which is capable of executing code one line at
    /// a time.
    // Also known as "REPL", from Lisp:
    // - Read a line of input
    // - Evaluate it
    // - Print the result
    // - Loop and do it all over again
    pub fn interactive() -> Result<(), MalisError> {
        let mut malis = Malis::default();
        // Get new handles to the stdin and stdout streams
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        // Create a new buffer to store the input
        let mut buffer = String::new();

        loop {
            // Write the new line identifier
            let _ = stdout.write(b"> ")?;
            // Flush it to make sure we print it
            stdout.flush()?;
            // Read the next line
            let bread = stdin.read_line(&mut buffer)?;

            // If no bytes were read, it means we reached `End-of-File` or `Ctrl-D` was pressed.
            if bread == 0 {
                break;
            }

            match buffer.as_str().trim() {
                "q" | "quit" | "exit" => break,
                _ => {}
            }

            // If a line is invalid, we report the error and go to the next iteration. We also
            // specify the `is_repl` true such that we could evaluate both expressions and
            // statements
            if let Err(err) = malis.run(buffer.as_str(), true) {
                println!("Interpreter: {err}");
                stdout.flush()?;
            }

            // Make sure to clean the buffer for the next iteration
            buffer.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{AstPrinter, Parser, Scanner};

    #[test]
    fn block_scope_test() {
        let file_path = "testdata/block_scope_test.ms";
        let source = std::fs::read_to_string(file_path).expect("Failed to read test file");

        let mut scanner = Scanner::new(source.as_str());
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        let mut parser = Parser::new(tokens);
        let expr = parser.parse().expect("Failed to parse tokens");
        let mut ast_printer = AstPrinter;

        assert!(
            format!("{}", ast_printer.print_stmt(&expr))
                == r#"(block scope (var decl (var a) LitString("global a")) (var decl (var b) LitString("global b")) (var decl (var c) LitString("global c")) (block scope (var decl (var a) LitString("outer a")) (var decl (var b) LitString("outer b")) (block scope (var decl (var a) LitString("inner a")) (print_stmt (var a)) (print_stmt (var b)) (print_stmt (var c))) (print_stmt (var a)) (print_stmt (var b)) (print_stmt (var c))) (print_stmt (var a)) (print_stmt (var b)) (print_stmt (var c)))"#
        );
    }
}
