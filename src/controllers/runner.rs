use super::{file::read_piggy_configuration_file, job_executor::execute};
use crate::prompts::{confirmation_yes, job_selection};

pub fn run(file_path: String) {
    let mut is_not_quit = true;
    let piggy = read_piggy_configuration_file(&file_path);

    while is_not_quit {
        let complete_project_folder = format!(
            "Project: {}\nConfiguration: {}",
            piggy.piggy_settings.project_folder_location,
            piggy.piggy_settings.configuration_file_location
        );
        println!("{}", complete_project_folder);

        let job = job_selection(&piggy.jobs);

        if confirmation_yes(&job.expect("No Job Selected").display_job_details()) {
            execute(job.expect("No Job Selected"), &piggy.piggy_settings)
        } else {
            // TODO do this on q/ Q keypress
            is_not_quit = false;
        }
    }
}
