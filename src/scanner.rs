mod token;

use crate::error::{ScannerError, SourceError};
use core::str::CharIndices;
use token::{Token, TokenType, SingleChar, Comparison};
use std::io::Write;
use core::iter::Peekable;

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
        let mut chars = self.data.char_indices().peekable();

        // Points to the first character in the lexeme being scanned.
        let mut start = 0;
        while let Some((idx, ch)) = chars.next() {
            self.offset = idx;
            start = self.offset;

            if let Ok(token) = self.scan_token(ch, start, &mut chars) {
                // If the current token is classified as `Ignored` we move to the next iteration
                if let Some(TokenType::Ignored) = token.t_type() {
                    continue;
                }
                // At this point, the token needs to be in the token list
                token_list.push(token);
            }
        }

        Ok(token_list)
    }

    // Note: I would prefer this being in `Token` and sending a slice of the data to a method in
    // `Token` where the first character would be the one at `self.offset`.
    pub fn scan_token(
        &mut self,
        ch: char,
        start: usize,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, ScannerError> {
        let token = match ch {
            '(' => self.create_token(TokenType::SingleChar(SingleChar::LeftParen), None, start)?,
            ')' => self.create_token(TokenType::SingleChar(SingleChar::RightParen), None, start)?,
            '{' => self.create_token(TokenType::SingleChar(SingleChar::LeftBrace), None, start)?,
            '}' => self.create_token(TokenType::SingleChar(SingleChar::RightBrace), None, start)?,
            ',' => self.create_token(TokenType::SingleChar(SingleChar::Comma), None, start)?,
            '.' => self.create_token(TokenType::SingleChar(SingleChar::Dot), None, start)?,
            '-' => self.create_token(TokenType::SingleChar(SingleChar::Minus), None, start)?,
            '+' => self.create_token(TokenType::SingleChar(SingleChar::Plus), None, start)?,
            ';' => self.create_token(TokenType::SingleChar(SingleChar::SemiColon), None, start)?,
            '!' => {
                if self.match_next('=', chars) {
                    self.create_token(TokenType::Comparison(Comparison::BangEqual), None, start)?
                } else {
                    self.create_token(TokenType::Comparison(Comparison::Bang), None, start)?
                }
            }
            '=' => {
                if self.match_next('=', chars) {
                    self.create_token(TokenType::Comparison(Comparison::EqualEqual), None, start)?
                } else {
                    self.create_token(TokenType::Comparison(Comparison::Equal), None, start)?
                }
            }
            '<' => {
                if self.match_next('=', chars) {
                    self.create_token(TokenType::Comparison(Comparison::LessEqual), None, start)?                } else {
                    self.create_token(TokenType::Comparison(Comparison::Less), None, start)?
                }
            }
            '>' => {
                if self.match_next('=', chars) {
                    self.create_token(TokenType::Comparison(Comparison::GreaterEqual), None, start)?
                } else {
                    self.create_token(TokenType::Comparison(Comparison::Greater), None, start)?
                }
            }
            '/' => {
                if self.match_next('/', chars) {
                    // A comment goes until the end of line. So we lookahead until we find
                    // a newline
                    while let Some((idx, peek_ch)) = chars.peek() {
                        // If we are not at the `newline` char, we consume the character.
                        if peek_ch != &'\n' { chars.next(); } else { break; }
                    }
                    self.create_token(TokenType::Ignored, None, start)?
                } else {
                    self.create_token(TokenType::SingleChar(SingleChar::Slash), None, start)?
                }
            }
            // Ignore whitespaces
            ' ' | '\r' | '\t' => {
                self.create_token(TokenType::Ignored, None, start)?
            }
            '\n' => {
                self.line += 1;
                self.create_token(TokenType::Ignored, None, start)?
            }
            _ => {
                let err = SourceError {
                    line: self.line,
                    location: start,
                    err: format!("Unexpected character: {ch}"),
                };
                let mut stdout = std::io::stdout();
                stdout.write_fmt(format_args!("{err:?}"))?;
                stdout.flush()?;

                return Err(ScannerError::UnexpectedCharacter(ch));
            }
        };
        Ok(token)
    }

    pub fn match_next(&mut self, expected: char, chars: &mut Peekable<CharIndices>) -> bool {
        if let Some((_idx, ch)) = chars.peek() {
            if ch == &expected {
                chars.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn create_token(
        &mut self,
        token_type: TokenType,
        literal: Option<bool>,
        start: usize,
    ) -> Result<Token, ScannerError> {
        let text = self.data.get(start..self.offset)
            .ok_or(ScannerError::FailedToIndexSlice)?
            .to_string();
        Ok(Token::new(token_type, text, literal, self.line))
    }

    pub fn next(&mut self) -> Result<u8, ScannerError> {
        Ok(0)
    }
}

