mod token;

use crate::error::ScannerError;
use core::str::CharIndices;
use token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    data: &'a str,
    // Current offset in the `data` field
    offset: usize,
    // The line cursor is on
    line: usize,
}

impl<'a> Scanner<'a> {
    // Creates a new scanner from the given bytes
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            offset: 0,
            line: 1,
        }
    }

    /// Scan through the internal buffer and issue `Token`s
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut token_list = vec![];
        let mut chars = self.data.char_indices();

        // Points to the first character in the lexeme being scanned.
        let mut start = 0;
        while let Some((idx, ch)) = chars.next() {
            self.offset = idx;
            start = self.offset;
            //self.scan_token(ch);
        }

        token_list.push(Token::new(TokenType::EOF, "", None, self.line));

        Ok(token_list)
    }

    // Note: I would prefer this being in `Token` and sending a slice of the data to a method in
    // `Token` where the first character would be the one at `self.offset`.
    pub fn scan_token(&mut self, ch: char) -> Result<Token, ScannerError> {
        todo!();
        //match ch {
        //    ')' => Token::new(TokenType::SingleChar(SingleChar::LeftParen), )
        //}
    }

    pub fn add_token(
        &mut self,
        token_type: TokenType,
        literal: Option<bool>,
    ) -> Result<Token, ScannerError> {
        todo!();
    }

    pub fn next(&mut self) -> Result<u8, ScannerError> {
        Ok(0)
    }
}
