
use crate::error::{ScannerError, SourceError};
use core::str::CharIndices;
use crate::token::{Token, TokenType, SingleChar, Comparison, Literal, Keyword};
use std::{
    io::Write,
    collections::HashMap,
};
use core::iter::Peekable;

#[derive(Debug)]
pub struct Scanner<'a> {
    data: &'a str,
    // Current offset in the `data` field
    offset: usize,
    // The line the cursor is on
    line: usize,
    // Keywords of the language
    keywords: HashMap<&'a str, Keyword>,
}

impl<'a> Scanner<'a> {
    // Creates a new scanner from the given bytes
    pub fn new(data: &'a str) -> Self {
        // We instantiate a dictionary for reserved words here, such that we save processing power
        // when we parse identifiers
        let keywords = HashMap::from([
            ("and", Keyword::And),
            ("or", Keyword::Or),
            ("not", Keyword::Not),
            ("class", Keyword::Class),
            ("fun", Keyword::Fun),
            ("if", Keyword::If),
            ("else", Keyword::Else),
            ("while", Keyword::While),
            ("for", Keyword::For),
            ("true", Keyword::True),
            ("false", Keyword::False),
            ("nil", Keyword::Nil),
            ("var", Keyword::Var),
            ("print", Keyword::Print),
            ("return", Keyword::Return),
        ]);
        Self {
            data,
            offset: 0,
            line: 1,
            keywords,
        }
    }

    /// Scan through the internal buffer and issue `Token`s
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut token_list = vec![];
        let mut error_list = vec![];
        let mut chars = self.data.char_indices().peekable();

        // Points to the first character in the lexeme being scanned.
        let mut start = 0;
        while let Some((idx, ch)) = chars.next() {
            self.offset = idx;
            start = self.offset;

            // Scan the next token
            let maybe_token = self.scan_token(ch, start, &mut chars);

            match maybe_token {
                Ok(token) => {
                    // If the current token is classified as `Ignored` we move to the next iteration
                    if let Some(TokenType::Ignored) = token.t_type() {
                        continue;
                    }
                    // At this point, the token needs to be in the token list
                    token_list.push(token);
                }
                // Add this error to our list of errors
                Err(err) => error_list.push(err),
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
            '(' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::LeftParen), start)?
            }
            ')' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::RightParen), start)?
            }
            '{' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::LeftBrace), start)?
            }
            '}' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::RightBrace), start)?
            }
            ',' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::Comma), start)?
            }
            '.' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::Dot), start)?
            }
            '-' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::Minus), start)?
            }
            '+' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::Plus), start)?
            }
            ';' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::SemiColon), start)?
            }
            '*' => {
                self.offset += 1;
                self.create_token(TokenType::SingleChar(SingleChar::SemiColon), start)?
            }
            '!' => {
                if self.match_next('=', chars) {
                    self.offset += 2;
                    self.create_token(TokenType::Comparison(Comparison::BangEqual), start)?
                } else {
                    self.offset += 1;
                    self.create_token(TokenType::Comparison(Comparison::Bang), start)?
                }
            }
            '=' => {
                if self.match_next('=', chars) {
                    self.offset += 2;
                    self.create_token(TokenType::Comparison(Comparison::EqualEqual), start)?
                } else {
                    self.offset += 1;
                    self.create_token(TokenType::Comparison(Comparison::Equal), start)?
                }
            }
            '<' => {
                if self.match_next('=', chars) {
                    self.offset += 2;
                    self.create_token(TokenType::Comparison(Comparison::LessEqual), start)?
                } else {
                    self.offset += 1;
                    self.create_token(TokenType::Comparison(Comparison::Less), start)?
                }
            }
            '>' => {
                if self.match_next('=', chars) {
                    self.offset += 2;
                    self.create_token(TokenType::Comparison(Comparison::GreaterEqual), start)?
                } else {
                    self.offset += 1;
                    self.create_token(TokenType::Comparison(Comparison::Greater), start)?
                }
            }
            '/' => {
                if self.match_next('/', chars) {
                    // A comment goes until the end of line. So we lookahead until we find
                    // a newline
                    while let Some((idx, peek_ch)) = chars.peek() {
                        // If we are not at the `newline` char, we consume the character.
                        if peek_ch != &'\n' {
                            self.offset = *idx;
                            chars.next();
                        } else { break; }
                    }
                    self.create_token(TokenType::Ignored, start)?
                } else if self.match_next('*', chars) {
                    // We do not allow multiline block comments to nest as it requires keeping
                    // a stack of previous open blocks characters `/*`
                    while let Some(&(idx, peek_ch)) = chars.peek() {
                        self.offset = idx;
                        chars.next();
                        if peek_ch == '*' {
                            if let Some(&(idx2, peek_ch2)) = chars.peek() {
                                if peek_ch2 == '/' {
                                    self.offset = idx2;
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                    self.create_token(TokenType::Ignored, start)?
                } else {
                    self.create_token(TokenType::SingleChar(SingleChar::Slash), start)?
                }
            }
            // Ignore whitespaces
            ' ' | '\r' | '\t' => {
                self.create_token(TokenType::Ignored, start)?
            }
            '\n' => {
                self.line += 1;
                self.create_token(TokenType::Ignored, start)?
            }
            '\"' => {
                self.parse_string(start, chars)?
            }
            _ => {
                if ch.is_digit(10) {
                    self.parse_number(start, chars)?
                } else if ch.is_ascii_alphabetic() || ch == '_' {
                    self.parse_ident(start, chars)?
                } else {
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
            }
        };
        Ok(token)
    }

    pub fn match_next(&mut self, expected: char, chars: &mut Peekable<CharIndices>) -> bool {
        if let Some((idx, ch)) = chars.peek() {
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

    /// Parses a literal string into a token until it finds it's terminating double quote `"`
    ///
    /// # Errors
    ///
    /// If end of `chars` is reached and no ending double-quote is found, it returns an error
    pub fn parse_string(
        &mut self,
        start: usize,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, ScannerError> {
        // While there is a next character in `chars`
        while let Some((idx, peek_ch)) = chars.peek() {
            // Update our offset to the current index position
            self.offset = *idx;
            // If there is a newline, we increment our line as well
            if peek_ch == &'\n' { self.line += 1 }
            // If we find the next quote, we found the end of the `String`
            if peek_ch == &'\"' {
                // We make sure we take into account the last character
                self.offset +=1;
                // Consume the final character of the literal string
                chars.next();
                break;
            }
            // Consume the current peeked character to advance
            chars.next();

            if self.offset == self.data.len() {
                // If we are at the end and we did not end the string, return an error
                return Err(ScannerError::UnterminatedString);
            }
        }

        // Get the string, without the surrounding quotes. This is the lexeme
        let value = self.data.get(start+1..self.offset-1)
            .ok_or(ScannerError::FailedToIndexSlice)?
            .to_string();

        // Create a token and return it
        self.create_token(TokenType::Literal(Literal::Ident(value)), start)
    }

    /// Parse a floating-point compatible token from `start` using characters from the `chars`
    /// iterator. This function stores both intergers and floatings point as a `f32`
    ///
    /// # Errors
    ///
    /// Fails if the range for the integer is invalid in the underlying data
    pub fn parse_number(
        &mut self,
        start: usize,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, ScannerError> {
        'int_while: while let Some(&(idx, peek_ch)) = chars.peek() {
            // If the peeked character is a digit, consume it
            if peek_ch.is_digit(10) {
                chars.next();
                self.offset = idx;
                continue;
            } else if peek_ch == '.' {
                // Check if we have a fractional part
                // Consume the '.'
                chars.next();
                self.offset = idx;
                while let Some(&(idx2, peek_ch2)) = chars.peek() {
                    if peek_ch2.is_digit(10) {
                        self.offset = idx2;
                        chars.next();
                    } else {
                        // If there are no more digits left in the fractional part, we leave
                        break 'int_while;
                    }
                }
            } else {
                break;
            }
        }
        // Go to the next position
        self.offset += 1;

        let value = self.data.get(start..self.offset)
            .ok_or(ScannerError::FailedToIndexSlice)?;
        let value = value.parse()?;
        self.create_token(TokenType::Literal(Literal::Number(value)), start)
    }

    /// Parse an identifier (that could be languages reserved word) from the input. The identifier
    /// has to start with an ASCII alphabetic character or an underscore and could be composed of
    /// ASCII alphanumeric and undescore characters
    ///
    /// # Errors
    ///
    /// Fails if the bounds into the backing data are invalid and a string could not be created
    pub fn parse_ident(
        &mut self,
        start: usize,
        chars: &mut Peekable<CharIndices>,
    ) -> Result<Token, ScannerError> {
        while let Some((idx, ch)) = chars.peek() {
            if ch.is_ascii_alphanumeric() || *ch == '_' {
                self.offset = *idx;
                chars.next();
            } else {
                self.offset += 1;
                break;
            }
        }
        let value = self.data.get(start..self.offset)
            .ok_or(ScannerError::FailedToIndexSlice)?;

        // Check if the parsed token is a keyword for `Malis`
        let token_type = if let Some(keyword) = self.keywords.get(value) {
            TokenType::Keyword(*keyword)
        } else {
            TokenType::Literal(Literal::LitString(value.to_string()))
        };

        self.create_token(token_type, start)
    }

    pub fn create_token(
        &mut self,
        token_type: TokenType,
        start: usize,
    ) -> Result<Token, ScannerError> {
        let text = self.data.get(start..self.offset)
            .ok_or(ScannerError::FailedToIndexSlice)?
            .to_string();
        Ok(Token::new(token_type, text, self.line))
    }
}

