use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct PiggySettings {
    pub folder: String,
    pub configuration_file: String,
}