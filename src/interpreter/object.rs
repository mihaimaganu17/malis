use super::{
    Interpreter, MalisCallable, MalisClass, MalisInstance, NativeFunction, RuntimeError,
    UserFunction,
};
use core::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum MalisObject {
    Boolean(bool),
    Number(f32),
    StringValue(String),
    NativeFunction(Box<NativeFunction>),
    UserFunction(UserFunction),
    Class(MalisClass),
    Instance(MalisInstance),
    Nil,
}

impl fmt::Display for MalisObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::StringValue(value) => write!(f, "{value}"),
            Self::Nil => write!(f, "nil"),
            Self::Number(value) => write!(f, "{}", value),
            Self::NativeFunction(value) => write!(f, "<native fn {}>", value.name()),
            Self::UserFunction(value) => write!(f, "<fn {}>", value.name()),
            Self::Class(value) => write!(f, "<class {}>", value.name()),
            Self::Instance(value) => write!(f, "<class instance {}>", value.name()),
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
            // We consider function pointers as true
            MalisObject::NativeFunction(_)
            | MalisObject::UserFunction(_)
            | MalisObject::Class(_)
            | MalisObject::Instance(_) => true,
            // We consider null as false
            MalisObject::Nil => false,
        }
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, MalisObject::NativeFunction(_))
            || matches!(self, MalisObject::UserFunction(_))
            || matches!(self, MalisObject::Class(_))
    }
}

impl MalisCallable for MalisObject {
    fn arity(&self) -> Result<usize, RuntimeError> {
        match self {
            MalisObject::NativeFunction(f) => f.arity(),
            MalisObject::UserFunction(f) => f.arity(),
            MalisObject::Class(f) => f.arity(),
            _ => Err(RuntimeError::NotCallable(format!(
                "Object {} has no arity.",
                self
            ))),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        match self {
            MalisObject::NativeFunction(f) => f.call(interpreter, arguments),
            MalisObject::UserFunction(f) => f.call(interpreter, arguments),
            MalisObject::Class(f) => f.call(interpreter, arguments),
            _ => Err(RuntimeError::NotCallable(format!(
                "Object {} is not callable.",
                self
            ))),
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
            MalisObject::Boolean(false)
        } else {
            MalisObject::Boolean(true)
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
