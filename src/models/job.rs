use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub commands: Vec<String>,
}

impl Job {
    pub fn create_default_jobs() {

    }
    
    pub fn new(name: &str, commands: Vec<String>) {

    }
}