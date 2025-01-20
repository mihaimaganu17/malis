use crate::{
    error::{MalisError, AstError},
    token::{Token, TokenType, Keyword, Literal as LiteralToken},
    visit::Visitor,
};

pub enum Expr {
    Unary(Unary),
    Binary(Binary),
    Group(Group),
    Literal(Literal),
}

impl Expr {
    pub fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Group(group) => visitor.visit_group(group),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    pub l_type: LiteralType,
}

impl Literal {
    pub fn new(token: &Token) -> Result<Self, MalisError> {
        let l_type = if let Some(token) = token.t_type.get() {
            match token {
                TokenType::Literal(literal) => match literal {
                    LiteralToken::Number(value) => LiteralType::Number(*value),
                    LiteralToken::LitString(value) => LiteralType::LitString(value.clone()),
                    _ => Err(AstError::NotALiteral)?,
                }
                TokenType::Keyword(value) => match value {
                    Keyword::True => LiteralType::True,
                    Keyword::False => LiteralType::False,
                    Keyword::Nil => LiteralType::Nil,
                    _ => Err(AstError::NotALiteral)?,
                }
                _ => Err(AstError::NotALiteral)?,
            }
        } else {
            return Err(MalisError::NoneTokenType);
        };
        Ok(Self { l_type })
    }
}

#[derive(Debug)]
pub enum LiteralType {
    Number(f32),
    LitString(String),
    True,
    False,
    Nil,
}

// Grouping matches any expression derivation inside a parenthasis -> "(" expression ")"
pub struct Group {
    pub expr: Box<Expr>,
}

impl Group {
    pub fn new(expr: Expr) -> Self {
        Self { expr: Box::new(expr) }
    }
}
