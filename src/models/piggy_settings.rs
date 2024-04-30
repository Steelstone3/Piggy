use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PiggySettings {
    pub folder: String,
}

impl PiggySettings {
    pub fn update_folder(&mut self, folder: String) {
        self.folder = folder
    }
}
