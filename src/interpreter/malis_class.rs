use super::{Interpreter, MalisCallable, MalisObject, RuntimeError, UserFunction};
use crate::token::Token;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MalisClass {
    name: String,
    methods: BTreeMap<String, UserFunction>,
}

impl MalisClass {
    pub fn new(name: &str, methods: BTreeMap<String, UserFunction>) -> Self {
        Self {
            name: name.to_string(),
            methods,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get(&self, name: &Token) -> Result<UserFunction, RuntimeError> {
        self.methods
            .get(name.lexeme())
            .ok_or(RuntimeError::PropertyNotPresent(format!(
                "Property {:?} not present in instance of class {:?}",
                name.lexeme(),
                self.name()
            )))
            .cloned()
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
        Self {
            class,
            fields: BTreeMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.class.name()
    }

    pub fn get(&self, key: &Token) -> Result<MalisObject, RuntimeError> {
        let maybe_value = self.fields.get(key.lexeme());
        // If the name is a property of the class, we should find it in the fields map
        if let Some(value) = maybe_value {
            Ok(value.clone())
        } else {
            // Otherwise we want to check if the key does not refer to a class method
            Ok(MalisObject::UserFunction(self.class.get(key)?))
        }
    }

    // Set the property identified by `key` to `value`
    pub fn set(&mut self, key: &Token, value: MalisObject) -> Result<MalisObject, RuntimeError> {
        self.fields.insert(key.lexeme().to_string(), value.clone());
        Ok(value)
    }
}
