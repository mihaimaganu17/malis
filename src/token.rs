//! Defines and manipulates source code tokens
use core::cell::OnceCell;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    // Token type, `type` is reserved
    pub t_type: OnceCell<TokenType>,
    // Substring from the source code from which the token was parsed.
    lexeme: String,
    // Line on which the token occurs
    pub line: OnceCell<usize>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?}", self.t_type.get(), self.lexeme())
    }
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, line: usize) -> Self {
        // Technically this is an initializer so there is no possible way of the unwrap to fail.
        // (There are ways, but you have to try really hard)
        let t_type_cell = OnceCell::new();
        t_type_cell.set(t_type).unwrap();
        let line_cell = OnceCell::new();
        line_cell.set(line).unwrap();
        Self {
            t_type: t_type_cell,
            lexeme,
            line: line_cell,
        }
    }

    pub fn t_type(&self) -> Option<&TokenType> {
        self.t_type.get()
    }

    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }

    pub fn create(new_t_type: TokenType, new_lexeme: &str) -> Self {
        let t_type = OnceCell::new();
        t_type.set(new_t_type).expect("Failed to set token");
        let lexeme = new_lexeme.to_string();
        Self {
            t_type,
            lexeme,
            line: OnceCell::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    SingleChar(SingleChar),
    Comparison(Comparison),
    Literal(Literal),
    Keyword(Keyword),
    Ident,
    Ignored,
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Comparison {
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    // Because `String` is reserved in Rust
    LitString(String),
    Number(f32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
