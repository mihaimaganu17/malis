use crate::interpreter::MalisObject;
use std::collections::HashMap;
use std::rc::Weak;
use std::cell:: RefCell;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, MalisObject>,
    // Weak reference to the parent environment of this environment. The global environment has this
    // value None
    enclosing: Option<Weak<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Weak<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(
        &mut self,
        name: String,
        value: MalisObject,
    ) -> Result<Option<MalisObject>, EnvironmentError> {
        Ok(self.values.insert(name, value))
    }

    pub fn get(&self, name: &str) -> Result<&MalisObject, EnvironmentError> {
        self.values
            .get(name)
            .ok_or(EnvironmentError::UndefinedVariable(name.to_string()))
    }

    pub fn insert(&mut self, name: &str, value: MalisObject) -> Result<MalisObject, EnvironmentError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone()).unwrap();
            Ok(value)
        } else {
            Err(EnvironmentError::UndefinedVariable(name.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentError {
    UndefinedVariable(String),
}
