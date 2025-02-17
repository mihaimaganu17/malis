use super::{Interpreter, MalisObject, RuntimeError};
use crate::{ast::FunctionDeclaration, token::Token};
use core::cmp::Ordering;
use std::fmt;

pub trait MalisCallable {
    fn arity(&self) -> Result<usize, RuntimeError>;

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError>;
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct NativeFunction {
    name: String,
    arity: usize,
    call_fn: fn(&mut Interpreter, Vec<MalisObject>) -> Result<MalisObject, RuntimeError>,
}

impl NativeFunction {
    pub fn new(
        name: String,
        arity: usize,
        call_fn: fn(&mut Interpreter, Vec<MalisObject>) -> Result<MalisObject, RuntimeError>,
    ) -> Self {
        Self {
            name,
            arity,
            call_fn,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl MalisCallable for NativeFunction {
    fn arity(&self) -> Result<usize, RuntimeError> {
        Ok(self.arity)
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        (self.call_fn)(interpreter, arguments)
    }
}

impl MalisCallable for Box<NativeFunction> {
    fn arity(&self) -> Result<usize, RuntimeError> {
        Ok(self.arity)
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        (self.call_fn)(interpreter, arguments)
    }
}

#[derive(Clone)]
pub struct UserFunction {
    function_declaration: FunctionDeclaration,
}

impl UserFunction {
    pub fn new(function_declaration: FunctionDeclaration) -> Self {
        UserFunction {
            function_declaration,
        }
    }

    pub fn name(&self) -> &Token {
        &self.function_declaration.name
    }
}

impl MalisCallable for UserFunction {
    fn arity(&self) -> Result<usize, RuntimeError> {
        Ok(self.function_declaration.parameters.len())
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        // Create a new environment that encapsulates the parameters
        let mut environment = interpreter.environment.take();
        // Define all the parameters of the function in the new environment
        for (param, arg) in self
            .function_declaration
            .parameters
            .iter()
            .zip(arguments.into_iter())
        {
            environment.define(param.lexeme().to_string(), arg)?;
        }

        // With the new environment defined, execute the body of the function
        match interpreter.execute_block(&self.function_declaration.body, environment) {
            Ok(_) => Ok(MalisObject::Nil),
            Err(RuntimeError::Return(return_obj)) => Ok(return_obj),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Debug for UserFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<fn {}> (", self.function_declaration.name)?;

        for param in &self.function_declaration.parameters {
            write!(f, "{},", param)?;
        }

        writeln!(f, ")")
    }
}

impl PartialEq for UserFunction {
    fn eq(&self, other: &Self) -> bool {
        if self.function_declaration.name != other.function_declaration.name {
            return false;
        }

        if self.function_declaration.parameters.len() != other.function_declaration.parameters.len()
        {
            return false;
        }

        for (params, other_params) in self
            .function_declaration
            .parameters
            .iter()
            .zip(other.function_declaration.parameters.iter())
        {
            if params != other_params {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for UserFunction {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        // We do not order functions, but we need to implement this trait in order to confirm to
        // the `MalisObject` protocol
        None
    }
}
