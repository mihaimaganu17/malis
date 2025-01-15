use crate::ast::{Expr, Unary, Binary, Literal, Grouping};

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
}
