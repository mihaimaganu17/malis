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

    pub fn get(&self, name: &str) -> Result<UserFunction, RuntimeError> {
        self.methods
            .get(name)
            .ok_or(RuntimeError::PropertyNotPresent(format!(
                "Property {:?} not present in instance of class {:?}",
                name,
                self.name()
            )))
            .cloned()
    }
}

impl MalisCallable for MalisClass {
    fn arity(&self) -> Result<usize, RuntimeError> {
        // If we have an initializer method present, the arity is represented by the number of
        // arguments for the init method
        if let Ok(method) = self.get("init") {
            method.arity()
        } else {
            Ok(0)
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<MalisObject>,
    ) -> Result<MalisObject, RuntimeError> {
        // Create a new instance for the class
        let instance = MalisInstance::new(self.clone());
        // Find the init method and call it to initialise the instance
        if let Ok(method) = self.get("init") {
            // Bind the method to the current instance and call it.
            let object = method.bind(&instance)?.call(interpreter, arguments)?;
            if let MalisObject::Instance(_) = object {
                Ok(object)
            } else {
                panic!();
            }
        } else {
            // The object returned by init has to be an instance of the same class type
            Ok(MalisObject::Instance(instance))
        }
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
            let method = self.class.get(key.lexeme())?;
            Ok(MalisObject::UserFunction(method.bind(self)?))
        }
    }

    // Set the property identified by `key` to `value`
    pub fn set(&mut self, key: &Token, value: MalisObject) -> Result<MalisObject, RuntimeError> {
        self.fields.insert(key.lexeme().to_string(), value.clone());
        Ok(value)
    }
}
