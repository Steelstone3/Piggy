use homedir::get_my_home;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PiggySettings {
    pub project_folder_location: String,
    pub configuration_file_location: String,
}

impl Default for PiggySettings {
    fn default() -> Self {
        let mut project_folder_location = "".to_string();

        if get_my_home().unwrap().unwrap().as_path().exists() {
            project_folder_location = get_my_home()
                .unwrap()
                .unwrap()
                .as_path()
                .to_string_lossy()
                .into_owned()
        }

        Self {
            project_folder_location,
            configuration_file_location: "piggy.json".to_string(),
        }
    }
}

#[cfg(test)]
mod piggy_settings_should {

    use super::*;

    #[test]
    fn create_default() {
        // Given
        let expected_piggy_settings = PiggySettings {
            project_folder_location: "/home/user".to_string(),
            configuration_file_location: "piggy.json".to_string(),
        };

        // When
        let piggy_settings = PiggySettings::default();

        // Then
        assert_eq!(
            expected_piggy_settings.configuration_file_location,
            piggy_settings.configuration_file_location
        );
    }
}
