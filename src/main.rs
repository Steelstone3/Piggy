use controllers::{
    file::{
        create_default_piggy_configuration_file, is_existing_file, read_piggy_configuration_file,
    },
    job_executor::execute,
};
use prompts::{confirmation, job_selection, text_prompt};

mod controllers;
mod models;
mod prompts;

pub fn main() {
    let mut file_path = "piggy.json".to_string();

    if confirmation("Use default piggy.toml file?") {
        if !is_existing_file(&file_path) {
            create_default_piggy_configuration_file()
        }
    } else {
        file_path = "".to_string();

        while !is_existing_file(&file_path) {
            file_path = text_prompt(
                "Specify piggy.toml file path",
                "Where is piggy.json on your system?",
                "./piggy.json",
            );
        }
    }

    let piggy = read_piggy_configuration_file(&file_path);
    let job = job_selection(piggy.jobs);
    execute(&job, &piggy.piggy_settings)
}
