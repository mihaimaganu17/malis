mod error;
mod scanner;

pub use error::MalisError;
use std::{fs, io::{self, Write}, path::Path};
use scanner::Scanner;

#[derive(Debug)]
pub struct Malis {
    // Keeps track of wheather the code has an error and to avoid executing it.
    had_error: bool,
}

impl Malis {
    pub fn execute<P: AsRef<Path>>(path: P) -> Result<(), MalisError> {
        let source = fs::read_to_string(path)?;
        Malis::run(source.as_str())
    }

    pub fn run<'a>(bytes: &'a str) -> Result<(), MalisError> {
        let mut scanner = Scanner::new(bytes);
        let tokens = scanner.scan_tokens()?;

        for token in tokens {
            println!("Token: {token:?}");
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
        // Get a new handle to the stdin and stdout
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        // Create a new buffer to store the input
        let mut buffer = String::new();

        loop {
            // Write the new line identifier
            stdout.write(b"> ")?;
            // Flush it to make sure we print it
            stdout.flush()?;
            // Read the next line
            let bread = stdin.read_line(&mut buffer)?;

            // If no bytes were read, it means we reached `End-of-File` or `Ctrl-D` was pressed.
            if bread == 0 {
                break;
            }

            // If a line is invalid, we report the error and go to the next iteration
            if let Err(err) = Self::run(buffer.as_str()) {
                print!("{:?}", err);
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
}
