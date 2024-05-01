use crate::prompts::{confirmation_yes, text_prompt};
use super::file::{create_default_piggy_configuration_file, is_existing_file};

pub fn find_configuration_file(file_path: &mut String) {
    if confirmation_yes("Use default piggy.toml file?") {
        if !is_existing_file(&*file_path) {
            create_default_piggy_configuration_file()
        }
    } else {
        *file_path = "".to_string();

        while !is_existing_file(&*file_path) {
            *file_path = text_prompt(
                "Specify piggy.toml file path",
                "Where is piggy.json on your system?",
                "./piggy.json",
            );
        }
    }
}
