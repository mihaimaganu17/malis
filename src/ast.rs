use crate::{
    error::{MalisError, AstError},
    token::{Token, TokenType, Keyword, Literal as LiteralToken},
    visit::Visitor,
};

pub trait Expr {
    fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T;
}

pub struct Unary<E: Expr> {
    operator: Token,
    right: E,
}

impl<E: Expr> Expr for Unary<E> {
    fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_unary(&self)
    }
}

impl<E: Expr> Unary<E> {
    pub fn new(operator: Token, right: E) -> Self {
        Self {
            operator,
            right,
        }
    }
}

pub struct Binary<E: Expr> {
    left: E,
    operator: Token,
    right: E,
}

impl<E: Expr> Binary<E> {
    fn new(left: E, operator: Token, right: E) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl<E: Expr> Expr for Binary<E> {
    fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_binary(&self)
    }
}

pub struct Literal {
    l_type: LiteralType,
}

impl Expr for Literal {
    fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_literal(&self)
    }
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

pub enum LiteralType {
    Number(f32),
    LitString(String),
    True,
    False,
    Nil,
}

// Grouping matches any expression derivation inside a parenthasis -> "(" expression ")"
pub struct Grouping<E: Expr> {
    expr: E,
}

impl<E: Expr> Grouping<E> {
    pub fn new(expr: E) -> Self {
        Self { expr }
    }
}

impl<E: Expr> Expr for Grouping<E> {
    fn walk<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        visitor.visit_grouping(&self)
    }
}
