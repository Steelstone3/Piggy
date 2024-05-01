use controllers::{
    file::{
        create_default_piggy_configuration_file, is_existing_file, read_piggy_configuration_file,
    },
    job_executor::execute,
};
use prompts::{confirmation_yes, job_selection, text_prompt};

mod controllers;
mod models;
mod prompts;

pub fn main() {
    let mut file_path = "piggy.json".to_string();

    if confirmation_yes("Use default piggy.toml file?") {
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

    // TODO work out how to loop this here
    // let mut is_not_quit = true;
    // while is_not_quit {
    loop {
        let job = job_selection(&piggy.jobs);
        if confirmation_yes(&job.expect("No Job Selected").display_job_details()) {
            execute(job.expect("No Job Selected"), &piggy.piggy_settings)
        }
    }
}

//  else {
//     // TODO do this on q/ Q keypress
//     exit(0);

//     #[cfg(target_os = "windows")]
//     exit(256);
// }
