use crate::ast::{Expr, Unary, Binary, Literal, Grouping};

pub trait Visitor<T> {
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
}

#[derive(Debug)]
pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, unary: &Unary) -> String {
    }
}

pub fn walk_expr<E: Expr>(visitor: &mut Visitor, e: &Expr)
