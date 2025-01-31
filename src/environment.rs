use crate::interpreter::MalisObject;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, MalisObject>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
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
            Ok(self.values.insert(name.to_string(), value).unwrap())
        } else {
            Err(EnvironmentError::UndefinedVariable(name.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentError {
    UndefinedVariable(String),
}
