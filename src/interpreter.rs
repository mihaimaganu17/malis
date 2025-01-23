use crate::visit::Visitor;
use crate::token::{TokenType, SingleChar};
use crate::ast::{Unary, Binary, Ternary, Literal, LiteralType, Group};
use core::ops::{Neg, Not, Sub, Mul, Div};

#[derive(Debug)]
pub enum MalisObject {
    Boolean(bool),
    Number(f32),
    StringValue(String),
    Nil,
}

impl MalisObject {
    // Decides whether a `MalisObject` value is true or not inside the context of the `Malis`
    // language
    pub fn is_truthy(&self) -> bool {
        match self {
            MalisObject::Boolean(b) => *b,
            // We consider any value coming from a literal as true. What do we do about
            // 0?
            MalisObject::StringValue(_)
            | MalisObject::Number(_) => true,
            // We consider null as false
            MalisObject::Nil => false,
        }
    }
}

impl From<bool> for MalisObject {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl Not for MalisObject {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_truthy() {
            MalisObject::Boolean(true)
        } else {
            MalisObject::Boolean(false)
        }
    }
}

impl Neg for MalisObject {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let MalisObject::Number(n) = self {
            MalisObject::Number(-n)
        } else {
            panic!("Cannot minus object {:?}", self);
        }
    }
}

impl Sub for MalisObject {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = if let MalisObject::Number(n) = self {
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        let right = if let MalisObject::Number(n) = rhs {
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        MalisObject::Number(left - right)
    }
}

impl Mul for MalisObject {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = if let MalisObject::Number(n) = self {
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        let right = if let MalisObject::Number(n) = rhs {
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        MalisObject::Number(left * right)
    }
}

impl Div for MalisObject {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let left = if let MalisObject::Number(n) = self {
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        let right = if let MalisObject::Number(n) = rhs {
            if n == 0.0 {
                panic!("Zero is an invalid denominator!");
            }
            n
        } else {
            panic!("Cannot minus object {:?}", self);
        };
        MalisObject::Number(left / right)
    }
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
                TokenType::SingleChar(SingleChar::Minus) => -right_malis_object,
                TokenType::SingleChar(SingleChar::Bang) => !right_malis_object,
                _ => panic!("Invalid unary operator {:?}", unary.operator),
            }
        } else {
            panic!("Unary operator {:?} has not TokenType {:?}", unary.operator.lexeme, unary.operator.line);
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> MalisObject {
        // In a binary expression, we evaluate the operand from left to right and then evaulte
        // the binary expression itself
        let left_object = binary.left.walk(self);
        let right_object = binary.right.walk(self);

        if let Some(operator_type) = binary.operator.t_type.get() {
            match operator_type {
                TokenType::SingleChar(SingleChar::Minus) => left_object - right_object,
                TokenType::SingleChar(SingleChar::Slash) => left_object / right_object,
                TokenType::SingleChar(SingleChar::Star) => left_object * right_object,
                _ => panic!("Invalid binary operator {:?}", binary.operator),
            }
        } else {
            panic!("Binary operator {:?} has not TokenType {:?}", binary.operator.lexeme, binary.operator.line);
        }
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
            LiteralType::True => MalisObject::Boolean(true),
            LiteralType::False => MalisObject::Boolean(false),
            LiteralType::Nil => MalisObject::Nil,
        }
    }
    // Grouping is an expression surrounded by parenthesis. To evaluate the grouping expression
    // itself, we recursively evaluate the subexpression contained and return it.
    fn visit_group(&mut self, group: &Group) -> MalisObject {
        group.expr.walk(self)
    }
}
