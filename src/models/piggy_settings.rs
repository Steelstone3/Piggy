use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PiggySettings {
    pub folder: String,
    pub configuration_file: String,
}

impl PiggySettings {
    pub fn update_folder(&mut self, folder: String) {
        self.folder = folder
    }

    pub fn update_configuration_file(&mut self, configuration_file: String) {
        self.configuration_file = configuration_file
    }
}
