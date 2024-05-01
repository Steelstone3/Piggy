use crate::models::{job::Job, piggy_settings::PiggySettings};
use std::process::Command;

pub fn execute(job: &Job, piggy_settings: &PiggySettings) {
    let mut command = Command::new(&job.command);
    command.current_dir(&piggy_settings.project_folder_location);
    command.args(&job.arguments);
    let output = command.output().expect("Failed to execute job");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
