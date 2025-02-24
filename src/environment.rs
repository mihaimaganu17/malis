use crate::interpreter::MalisObject;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Debug)]
pub struct Environment {
    pub values: HashMap<String, MalisObject>,
    // Weak reference to the parent environment of this environment. The global environment has this
    // value None
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Clone for Environment {
    // Yes, this is very very idiotic, but it is the only way to replicate Java behaviour :)
    fn clone(&self) -> Self {
        let values = self.values.clone();
        let enclosing = self
            .enclosing
            .as_ref()
            .map(|enclosing| Rc::new(RefCell::new(enclosing.borrow().clone())));
        Environment { values, enclosing }
    }
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: MalisObject) -> Result<(), EnvironmentError> {
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

    // Get the object identified by `name` which lives at the `distance` environment up
    pub fn get_at(&self, distance: usize, name: &str) -> Result<MalisObject, EnvironmentError> {
        while distance > 1 {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow().get_at(distance-1, name);
            } else {
                return Err(EnvironmentError::InvalidDistance(distance));
            }
        }
        self.get(name)
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

    pub fn insert_at(
        &mut self,
        distance: usize,
        name: &str,
        value: MalisObject,
    ) -> Result<MalisObject, EnvironmentError> {
        while distance != 0 {
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow_mut().insert_at(distance-1, name, value);
            } else {
                return Err(EnvironmentError::InvalidDistance(distance));
            }
        }
        self.insert(name, value)
    }
}

#[derive(Debug)]
pub enum EnvironmentError {
    UndefinedVariable(String),
    OutOfScope(String),
    InvalidDistance(usize),
}
