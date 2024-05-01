use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub commands: Vec<String>,
}

impl Job {
    #[allow(dead_code)]
    pub fn create_default_jobs() {}
}
