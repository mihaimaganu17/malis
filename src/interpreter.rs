use crate::visit::Visitor;
use crate::ast::{Unary, Binary, Ternary, Literal, Group};

#[derive(Debug)]
pub enum MalisObject {
    Boolean(bool),
    Number(f32),
    MalisString(String),
    Nil,
}

pub struct Interpreter {}

impl Visitor<MalisObject> for Interpreter {
    fn visit_unary(&mut self, _unary: &Unary) -> MalisObject {
        return MalisObject::Nil;
    }
    fn visit_binary(&mut self, _binary: &Binary) -> MalisObject {
        return MalisObject::Nil;
    }
    fn visit_ternary(&mut self, _ternary: &Ternary) -> MalisObject {
        return MalisObject::Nil;
    }
    fn visit_literal(&mut self, _literal: &Literal) -> MalisObject {
        return MalisObject::Nil;
    }
    fn visit_group(&mut self, _group: &Group) -> MalisObject {
        return MalisObject::Nil;
    }
}
