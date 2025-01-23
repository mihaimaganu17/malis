use crate::visit::Visitor;
use crate::token::{TokenType, SingleChar};
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
    fn visit_unary(&mut self, unary: &Unary) -> MalisObject {
        // We first evaluate the operand expression;
        let right_malis_object = unary.right.walk(self);
        // Our interpreter is doing a post-order traversal - each node evaluates its children
        // before doing its own work. As such we first evaluated the underlying expression above
        // and now we are evaluating the operator of our current value
        if let Some(operator_type) = unary.operator.t_type.get() {
            match operator_type {
                TokenType::SingleChar(SingleChar::Minus) => {
                    if let MalisObject::Number(n) = right_malis_object {
                        MalisObject::Number(-n)
                    } else {
                        panic!("Cannot minus object {:?} at {:?}", right_malis_object, unary.operator.line);
                    }
                }
                TokenType::SingleChar(SingleChar::Bang) => {
                    match right_malis_object {
                        MalisObject::True => MalisObject::False,
                        MalisObject::False => MalisObject::True,
                        _ => panic!("Cannot negate object {:?} at {:?}", right_malis_object, unary.operator.line),
                    }
                }
                _ => panic!("Invalid unary operator {:?}", unary.operator),
            }
        } else {
            panic!("Unary operator {:?} has not TokenType {:?}", unary.operator.lexeme, unary.operator.line);
        }
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
    // Grouping is an expression surrounded by parenthesis. To evaluate the grouping expression
    // itself, we recursively evaluate the subexpression contained and return it.
    fn visit_group(&mut self, group: &Group) -> MalisObject {
        group.expr.walk(self)
    }
}
