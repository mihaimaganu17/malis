use std::collections::HashMap;
use crate::interpreter::MalisObject;

pub struct Environment {
    values: HashMap<String, MalisObject>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: MalisObject) -> Result<Option<MalisObject>, EnvironmentError> {
        Ok(self.values.insert(name, value))
    }
}

#[derive(Debug)]
pub enum EnvironmentError {}
