//! Defines and manipulates source code tokens
use core::cell::OnceCell;
use std::fmt;


#[derive(Debug)]
pub struct Token {
    // Token type, `type` is reserved
    t_type: OnceCell<TokenType>,
    // Substring from the source code from which the token was parsed.
    lexeme: OnceCell<String>,
    literal: OnceCell<Option<bool>>,
    line: OnceCell<usize>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} {:?} {:?}", self.t_type.get(), self.lexeme.get(), self.literal.get())
    }
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Option<bool>, line: usize) -> Self {
        // Technically this is an initializer so there is no possible way of the unwrap to fail.
        // (There are ways, but you have to try really hard)
        let t_type_cell = OnceCell::new();
        t_type_cell.set(t_type).unwrap();
        let lexeme_cell = OnceCell::new();
        lexeme_cell.set(lexeme).unwrap();
        let literal_cell = OnceCell::new();
        literal_cell.set(literal).unwrap();
        let line_cell = OnceCell::new();
        line_cell.set(line).unwrap();
        Self {
            t_type: t_type_cell,
            lexeme: lexeme_cell,
            literal: literal_cell,
            line: line_cell,
        }
    }

    pub fn t_type(&self) -> Option<&TokenType> {
        self.t_type.get()
    }
}

#[derive(Debug)]
pub enum TokenType {
    SingleChar(SingleChar),
    Comparison(Comparison),
    Literal(Literal),
    Keyword(Keyword),
    Ignored,
    EOF,
}

#[derive(Debug)]
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
    Slash,
    Star,
}

#[derive(Debug)]
pub enum Comparison {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
pub enum Literal {
    Ident,
    // Because `String` is reserved in Rust
    LitString,
    Number,
}

#[derive(Debug)]
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
