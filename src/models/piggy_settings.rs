use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PiggySettings {
    pub folder_location: String,
    pub configuration_file_location: String,
}

impl PiggySettings {
    #[allow(dead_code)]
    pub fn new(folder_location: &str, configuration_file_location: &str) -> PiggySettings {
        Self {
            folder_location: folder_location.to_string(),
            configuration_file_location: configuration_file_location.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn new_using_default_configuration(folder_location: &str) -> PiggySettings {
        Self {
            folder_location: folder_location.to_string(),
            configuration_file_location: "".to_string(),
        }
    }
}

#[cfg(test)]
mod space_sprite_should {

    use super::*;

    #[test]
    fn create_new() {
        // Given
        let expected_piggy_settings = PiggySettings {
            folder_location: "example_1".to_string(),
            configuration_file_location: "example_2".to_string(),
        };

        // When
        let piggy_settings = PiggySettings::new("example_1", "example_2");

        // Then
        assert_eq!(expected_piggy_settings, piggy_settings);
    }

    #[test]
    fn create_new_using_default_configuration() {
        // Given
        let expected_piggy_settings = PiggySettings {
            folder_location: "example_1".to_string(),
            configuration_file_location: "".to_string(),
        };

        // When
        let piggy_settings = PiggySettings::new_using_default_configuration("example_1");

        // Then
        assert_eq!(expected_piggy_settings, piggy_settings);
    }
}
