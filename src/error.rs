use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub enum MalisError {
    StdIoError(std::io::Error),
    ScannerError(ScannerError),
    NoneTokenType,
    AstError(AstError),
    ParserError(ParserError),
    RuntimeError(RuntimeError),
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

impl From<RuntimeError> for MalisError {
    fn from(err: RuntimeError) -> Self {
        Self::RuntimeError(err)
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
        writeln!(
            f,
            "[line {0}] Error {1:?}: {2:?}",
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

#[derive(Debug)]
pub enum RuntimeError {
    Negation(String),
    Addition(String),
    Subtraction(String),
    Multiplication(String),
    Division(String),
    UnaryEvaluation(String),
    BinaryEvaluation(String),
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
