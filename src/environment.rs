use crate::interpreter::MalisObject;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Environment {
    values: HashMap<String, MalisObject>,
    // Weak reference to the parent environment of this environment. The global environment has this
    // value None
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: MalisObject) -> Result<(), EnvironmentError> {
        println!("Name {name}");
        self.values.insert(name, value);
        Ok(())
    }

    // Note: This is not ideal, as we clone the object when getting it. It would be ideal if the
    // storage was a reference and we could do a cheap clone of the object.
    pub fn get(&self, name: &str) -> Result<MalisObject, EnvironmentError> {
        let value_in_current_scope = self
            .values
            .get(name)
            .ok_or(EnvironmentError::UndefinedVariable(name.to_string()));

        if value_in_current_scope.is_ok() {
            value_in_current_scope.cloned()
        } else if let Some(enclosing) = &self.enclosing {
            Ok(enclosing.borrow().get(name)?)
        } else {
            Err(EnvironmentError::UndefinedVariable(name.to_string()))
        }
    }

    pub fn insert(
        &mut self,
        name: &str,
        value: MalisObject,
    ) -> Result<MalisObject, EnvironmentError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value.clone()).unwrap();
            return Ok(value);
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().insert(name, value);
        }

        Err(EnvironmentError::UndefinedVariable(name.to_string()))
    }
}

#[derive(Debug)]
pub enum EnvironmentError {
    UndefinedVariable(String),
    OutOfScope(String),
}
