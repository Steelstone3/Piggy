use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub commands: Vec<String>,
}

impl Job {
    pub fn create_default_jobs() -> Vec<Job> {
        vec![
            Job {
                name: "git-sync".to_string(),
                commands: vec![
                    "git".to_string(),
                    "add".to_string(),
                    "--all".to_string(),
                    "&&".to_string(),
                    "git".to_string(),
                    "commit".to_string(),
                    "-m update".to_string(),
                    "&&".to_string(),
                    "git".to_string(),
                    "pull".to_string(),
                    "--rebase".to_string(),
                    "&&".to_string(),
                    "git".to_string(),
                    "push".to_string(),
                ],
            },
            Job {
                name: "dotnet-test".to_string(),
                commands: vec!["dotnet".to_string(), "test".to_string()],
            },
            Job {
                name: "cargo-test".to_string(),
                commands: vec!["cargo".to_string(), "test".to_string()],
            },
        ]
    }
}
