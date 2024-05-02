use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub sub_folder: String,
    pub command: String,
    pub arguments: Vec<String>,
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Job {
    pub fn create_default_jobs() -> Vec<Job> {
        vec![
            Job {
                name: "dotnet-test".to_string(),
                sub_folder: "/ProjectTests".to_string(),
                command: "dotnet".to_string(),
                arguments: vec!["test".to_string()],
            },
            Job {
                name: "cargo-test".to_string(),
                sub_folder: "".to_string(),
                command: "cargo".to_string(),
                arguments: vec!["test".to_string()],
            },
        ]
    }

    pub fn display_job_details(&self) -> String {
        format!("Run: {} {}", self.command, self.arguments.join(" ")).to_string()
    }
}
