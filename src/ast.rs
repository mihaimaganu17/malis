use crate::token::Token;

pub trait Expr {}

pub struct Binary<E: Expr> {
    left: E,
    operator: Token,
    right: E,
}

impl<E: Expr> Binary<E> {
    pub fn new(left: E, operator: Token, right: E) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
