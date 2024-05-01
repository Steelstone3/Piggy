use controllers::file::{
    create_default_piggy_configuration_file, is_existing_file, read_piggy_configuration_file,
};
use models::piggy::Piggy;
use prompts::{confirmation, text_prompt};

mod controllers;
mod models;
mod prompts;

pub fn main() {
    let mut piggy = Piggy::default();

    if confirmation("Use default piggy.toml file?") {
        let file_path = "piggy.json".to_string();

        if !is_existing_file(&file_path)
            && confirmation(
                "Default piggy.toml file was not detected. Would you like to create one?",
            )
        {
            create_default_piggy_configuration_file()
        }

        piggy = read_piggy_configuration_file(&file_path);
    } else {
        let mut file_path = "".to_string();

        while !is_existing_file(&file_path) {
            file_path = text_prompt(
                "Specify piggy.toml file path",
                "Where is piggy.toml on your system?",
                ".",
            );
        }

        piggy = read_piggy_configuration_file(&file_path);
    }
}
