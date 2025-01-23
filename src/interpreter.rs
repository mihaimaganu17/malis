use crate::visit::Visitor;
use crate::ast::{Unary, Binary, Ternary, Literal, LiteralType, Group};

#[derive(Debug)]
pub enum MalisObject {
    Boolean(bool),
    Number(f32),
    StringValue(String),
    True,
    False,
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
    // Evaluating literals. A literal is a `bit of syntax` that produces a vlue. A literal
    // always appears somewhere in the user's source code. Lots of values are produces by
    // computation and don't exist anywhere in the code itself, but those are not literals.
    // A literal comes from the parser's domain. Values are an interpreter concept, part of the
    // runtime's world.
    fn visit_literal(&mut self, literal: &Literal) -> MalisObject {
        match &literal.l_type {
            LiteralType::Number(n) => MalisObject::Number(*n),
            LiteralType::LitString(s) => MalisObject::StringValue(s.to_string()),
            LiteralType::True => MalisObject::True,
            LiteralType::False => MalisObject::False,
            LiteralType::Nil => MalisObject::Nil,
        }
    }
    fn visit_group(&mut self, _group: &Group) -> MalisObject {
        return MalisObject::Nil;
    }
}
