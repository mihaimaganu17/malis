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
