#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoginHandle {
    name: String,
}

impl LoginHandle {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
