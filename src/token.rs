//! Defines and manipulates source code tokens
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Token {
    // Token type, `type` is reserved
    t_type: TokenType,
    // Substring from the source code from which the token was parsed.
    lexeme: String,
    // Line on which the token occurs
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?}", self.t_type, self.lexeme())
    }
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            t_type,
            lexeme,
            line,
        }
    }

    pub fn t_type(&self) -> &TokenType {
        &self.t_type
    }

    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn create(t_type: TokenType, new_lexeme: &str) -> Self {
        let lexeme = new_lexeme.to_string();
        Self {
            t_type,
            lexeme,
            line: 0,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TokenType {
    SingleChar(SingleChar),
    Comparison(Comparison),
    Literal(Literal),
    Keyword(Keyword),
    Ident,
    Ignored,
    Eof,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SingleChar {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Colon,
    Slash,
    Star,
    Bang,
    Question,
    Equal,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Comparison {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Literal {
    // Because `String` is reserved in Rust
    LitString(String),
    Number([u8; 4]),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Keyword {
    And,
    Or,
    Not,
    Class,
    Fun,
    If,
    Else,
    While,
    For,
    True,
    False,
    Nil,
    Var,
    Print,
    Return,
}
