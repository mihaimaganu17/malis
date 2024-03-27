use crate::error::ScannerError;

#[derive(Debug)]
pub struct Scanner;

impl Scanner {
    // Creates a new scanner from the given bytes
    pub fn new() -> Self {
        Self
    }

    /// Scan through the internal buffer and issue `Token`s
    pub fn scan_tokens(&self, bytes: &[u8]) -> Result<Vec<Token>, ScannerError> {
        Ok(vec![])
    }
}

// In example:
// ```
// var language = "lox";
// ```
// Each blob of characters like: `var`, `languages`, `=`, etc; is called a lexeme. 

#[derive(Debug)]
pub enum Token {}

