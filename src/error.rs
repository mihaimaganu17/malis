use crate::token::Token;
use crate::ast::Expr;
use std::fmt;

#[derive(Debug)]
pub enum MalisError {
    StdIoError(std::io::Error),
    ScannerError(ScannerError),
    NoneTokenType,
    AstError(AstError),
    ParserError(ParserError),
}

impl From<std::io::Error> for MalisError {
    fn from(err: std::io::Error) -> Self {
        Self::StdIoError(err)
    }
}

impl From<ScannerError> for MalisError {
    fn from(err: ScannerError) -> Self {
        Self::ScannerError(err)
    }
}

impl From<AstError> for MalisError {
    fn from(err: AstError) -> Self {
        Self::AstError(err)
    }
}

impl From<ParserError> for MalisError {
    fn from(err: ParserError) -> Self {
        Self::ParserError(err)
    }
}

#[derive(Debug)]
pub enum ScannerError {
    FailedToIndexSlice,
    StdIoError(std::io::Error),
    ParseFloatError(core::num::ParseFloatError),
    UnexpectedCharacter(char),
    UnterminatedString,
}

impl From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        Self::StdIoError(err)
    }
}

impl From<core::num::ParseFloatError> for ScannerError {
    fn from(err: core::num::ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

pub struct SourceError<P: fmt::Debug> {
    pub line: usize,
    pub location: usize,
    pub err: P,
}

impl<P: fmt::Debug> fmt::Debug for SourceError<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "[line {0}] Error {1:?}: {2:?}\n",
            self.line, self.location, self.err
        )
    }
}

#[derive(Debug)]
pub enum AstError {
    NotALiteral,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidIdx(usize),
    NegativeIdx,
    NoTokenType,
    MissingClosingParen,
    MissingColon,
    NoPrimaryProduction,
    NoErrorProduction,
    PanicMode(String, Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ParserError::PanicMode(message, token) => {
                write!(
                    f,
                    "Error on line {} for {}: {:#?}",
                    token.line.get().unwrap(),
                    token.lexeme.get().unwrap(),
                    message
                )
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
