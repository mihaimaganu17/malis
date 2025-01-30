pub mod ast;
mod environment;
mod error;
pub mod interpreter;
mod parser;
mod scanner;
mod token;
mod visit;

pub use error::MalisError;
use interpreter::Interpreter;
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
        malis.run(source.as_str())
    }

    pub fn run(&mut self, bytes: &str) -> Result<(), MalisError> {
        let mut scanner = Scanner::new(bytes);
        let maybe_tokens = scanner.scan_tokens();

        match maybe_tokens {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                let expr = parser.parse()?;
                let mut ast_printer = AstPrinter;
                println!("Ast: {}", ast_printer.print_stmt(&expr));

                self.interpreter.interpret(expr.as_slice())?;
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

            // If a line is invalid, we report the error and go to the next iteration
            if let Err(err) = malis.run(buffer.as_str()) {
                println!("{err}");
                stdout.flush()?;
            }

            // Make sure to clean the buffer for the next iteration
            buffer.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
