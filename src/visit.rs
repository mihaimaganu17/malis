use crate::ast::{Expr, Unary, Binary, Literal, Grouping};

pub trait Visitor<T> {
    fn visit_unary<E: Expr>(&mut self, unary: &Unary<E>) -> T;
    fn visit_binary<E: Expr>(&mut self, binary: &Binary<E>) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_grouping<E: Expr>(&mut self, grouping: &Grouping<E>) -> T;
}

#[derive(Debug)]
pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_unary<E: Expr>(&mut self, unary: &Unary<E>) -> String {
        String::new()
    }
    fn visit_binary<E: Expr>(&mut self, binary: &Binary<E>) -> String {
        String::new()
    }
    fn visit_literal(&mut self, literal: &Literal) -> String {
        String::new()
    }
    fn visit_grouping<E: Expr>(&mut self, grouping: &Grouping<E>) -> String {
        String::new()
    }
}
