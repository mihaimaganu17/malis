use crate::ast::{Binary, Expr, Group, Literal, LiteralType, Ternary, Unary};
use crate::error::RuntimeError;
use crate::token::{Comparison, SingleChar, TokenType};
use crate::visit::Visitor;
use core::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MalisObject {
    Boolean(bool),
    Number(f32),
    StringValue(String),
    Nil,
}

impl fmt::Display for MalisObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::StringValue(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(value) => write!(f, "{}", value),
        }
    }
}

impl MalisObject {
    // Decides whether a `MalisObject` value is true or not inside the context of the `Malis`
    // language
    pub fn is_truthy(&self) -> bool {
        match self {
            MalisObject::Boolean(b) => *b,
            // We consider any value coming from a literal as true. What do we do about
            // 0?
            MalisObject::StringValue(_) | MalisObject::Number(_) => true,
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
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Self::Output {
        if let MalisObject::Number(n) = self {
            Ok(MalisObject::Number(-n))
        } else {
            Err(RuntimeError::Negation(format!(
                "Cannot negate object {:?}",
                self
            )))
        }
    }
}

impl Add for MalisObject {
    type Output = Result<Self, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            MalisObject::Number(left) => match rhs {
                MalisObject::Number(right) => Ok(MalisObject::Number(left + right)),
                MalisObject::StringValue(right) => {
                    Ok(MalisObject::StringValue(format!("{left}{right}")))
                }
                _ => Err(RuntimeError::Addition(format!(
                    "Cannot add objects {:?} and {:?}",
                    self, rhs
                ))),
            },
            MalisObject::StringValue(ref left) => match rhs {
                MalisObject::StringValue(right) => {
                    Ok(MalisObject::StringValue(format!("{left}{right}")))
                }
                MalisObject::Number(right) => {
                    Ok(MalisObject::StringValue(format!("{left}{right}")))
                }
                _ => Err(RuntimeError::Addition(format!(
                    "Cannot add objects {:?} and {:?}",
                    self, rhs
                ))),
            },
            _ => Err(RuntimeError::Addition(format!(
                "Cannot add objects {:?} and {:?}",
                self, rhs
            ))),
        }
    }
}

impl Sub for MalisObject {
    type Output = Result<Self, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let MalisObject::Number(left) = self {
            if let MalisObject::Number(right) = rhs {
                Ok(MalisObject::Number(left - right))
            } else {
                Err(RuntimeError::Subtraction(format!(
                    "Cannot subtract objects {:?} and {:?}",
                    self, rhs
                )))
            }
        } else {
            Err(RuntimeError::Subtraction(format!(
                "Cannot subtract objects {:?} and {:?}",
                self, rhs
            )))
        }
    }
}

impl Mul for MalisObject {
    type Output = Result<Self, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let MalisObject::Number(left) = self {
            if let MalisObject::Number(right) = rhs {
                Ok(MalisObject::Number(left * right))
            } else {
                Err(RuntimeError::Multiplication(format!(
                    "Cannot multiply objects {:?} and {:?}",
                    self, rhs
                )))
            }
        } else {
            Err(RuntimeError::Multiplication(format!(
                "Cannot multiply objects {:?} and {:?}",
                self, rhs
            )))
        }
    }
}

impl Div for MalisObject {
    type Output = Result<Self, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        if let MalisObject::Number(left) = self {
            if let MalisObject::Number(right) = rhs {
                if right == 0.0 {
                    Err(RuntimeError::Division(format!(
                        "Zero is an invalid denominator {:?}!",
                        right
                    )))
                } else {
                    Ok(MalisObject::Number(left / right))
                }
            } else {
                Err(RuntimeError::Division(format!(
                    "Cannot divide objects {:?} and {:?}",
                    self, rhs
                )))
            }
        } else {
            Err(RuntimeError::Division(format!(
                "Cannot divide objects {:?} and {:?}",
                self, rhs
            )))
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, expr: Expr) -> Result<(), RuntimeError> {
        let malis_object = self.evaluate(expr)?;
        println!("{}", malis_object);
        Ok(())
    }

    pub fn evaluate(&mut self, expr: Expr) -> Result<MalisObject, RuntimeError> {
        expr.walk(self)
    }
}

impl Visitor<Result<MalisObject, RuntimeError>> for Interpreter {
    fn visit_unary(&mut self, unary: &Unary) -> Result<MalisObject, RuntimeError> {
        // We first evaluate the operand expression;
        let right_malis_object = unary.right.walk(self)?;
        // Our interpreter is doing a post-order traversal - each node evaluates its children
        // before doing its own work. As such we first evaluated the underlying expression above
        // and now we are evaluating the operator of our current value
        if let Some(operator_type) = unary.operator.t_type.get() {
            match operator_type {
                TokenType::SingleChar(SingleChar::Minus) => -right_malis_object,
                TokenType::SingleChar(SingleChar::Bang) => Ok(!right_malis_object),
                _ => Err(RuntimeError::UnaryEvaluation(format!(
                    "Invalid unary operator {:?}",
                    unary.operator
                ))),
            }
        } else {
            Err(RuntimeError::UnaryEvaluation(format!(
                "Unary operator {:?} has not TokenType {:?}",
                unary.operator.lexeme, unary.operator.line
            )))
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> Result<MalisObject, RuntimeError> {
        // In a binary expression, we evaluate the operand from left to right and then evaulte
        // the binary expression itself
        let left_object = binary.left.walk(self)?;
        let right_object = binary.right.walk(self)?;

        if let Some(operator_type) = binary.operator.t_type.get() {
            match operator_type {
                TokenType::SingleChar(SingleChar::Plus) => left_object + right_object,
                TokenType::SingleChar(SingleChar::Minus) => left_object - right_object,
                TokenType::SingleChar(SingleChar::Slash) => left_object / right_object,
                TokenType::SingleChar(SingleChar::Star) => left_object * right_object,
                TokenType::Comparison(Comparison::Greater) => {
                    Ok(MalisObject::Boolean(left_object.gt(&right_object)))
                }
                TokenType::Comparison(Comparison::GreaterEqual) => {
                    Ok(MalisObject::Boolean(left_object.ge(&right_object)))
                }
                TokenType::Comparison(Comparison::Less) => {
                    Ok(MalisObject::Boolean(left_object.lt(&right_object)))
                }
                TokenType::Comparison(Comparison::LessEqual) => {
                    Ok(MalisObject::Boolean(left_object.le(&right_object)))
                }
                TokenType::Comparison(Comparison::BangEqual) => {
                    Ok(MalisObject::Boolean(left_object.ne(&right_object)))
                }
                TokenType::Comparison(Comparison::EqualEqual) => {
                    Ok(MalisObject::Boolean(left_object.eq(&right_object)))
                }
                _ => Err(RuntimeError::BinaryEvaluation(format!(
                    "Invalid binary operator {:?}",
                    binary.operator
                ))),
            }
        } else {
            Err(RuntimeError::BinaryEvaluation(format!(
                "Binary operator {:?} has not TokenType {:?}",
                binary.operator.lexeme, binary.operator.line
            )))
        }
    }
    fn visit_ternary(&mut self, _ternary: &Ternary) -> Result<MalisObject, RuntimeError> {
        Ok(MalisObject::Nil)
    }
    // Evaluating literals. A literal is a `bit of syntax` that produces a vlue. A literal
    // always appears somewhere in the user's source code. Lots of values are produces by
    // computation and don't exist anywhere in the code itself, but those are not literals.
    // A literal comes from the parser's domain. Values are an interpreter concept, part of the
    // runtime's world.
    fn visit_literal(&mut self, literal: &Literal) -> Result<MalisObject, RuntimeError> {
        let malis_object = match &literal.l_type {
            LiteralType::Number(n) => MalisObject::Number(*n),
            LiteralType::LitString(s) => MalisObject::StringValue(s.to_string()),
            LiteralType::True => MalisObject::Boolean(true),
            LiteralType::False => MalisObject::Boolean(false),
            LiteralType::Nil => MalisObject::Nil,
        };
        Ok(malis_object)
    }
    // Grouping is an expression surrounded by parenthesis. To evaluate the grouping expression
    // itself, we recursively evaluate the subexpression contained and return it.
    fn visit_group(&mut self, group: &Group) -> Result<MalisObject, RuntimeError> {
        group.expr.walk(self)
    }
}
