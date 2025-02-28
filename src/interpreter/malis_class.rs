use super::{MalisCallable, RuntimeError, MalisObject, Interpreter};
use crate::token::Token;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MalisClass {
    name: String,
}

impl MalisClass {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl MalisCallable for MalisClass {
    fn arity(&self) -> Result<usize, RuntimeError> {
        Ok(0)
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        let instance = MalisInstance::new(self.clone());
        Ok(MalisObject::Instance(instance))
    }
}

/// Represents an insobject tance of the `MalisClass`
// Every instance is an open collection of named values. Methods on the instance's class can access
// and modify properties, but so can outside code.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MalisInstance {
    class: MalisClass,
    // Each field in this class intance has a property name (key in the map) and a propery value
    fields: BTreeMap<String, MalisObject>,
}

impl MalisInstance {
    pub fn new(class: MalisClass) -> Self {
        Self { class, fields: BTreeMap::new() }
    }

    pub fn name(&self) -> &str {
        &self.class.name()
    }

    pub fn get(&self, key: &Token) -> Result<MalisObject, RuntimeError> {
        self.fields.get(key.lexeme())
            .ok_or(RuntimeError::PropertyNotPresent(format!("Property {:?} not present in instance of class {:?}", key.lexeme(), self.class.name()))).cloned()
    }
}
