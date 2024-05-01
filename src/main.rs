use controllers::{
    file_finder::find_configuration_file, runner::run
};

mod controllers;
mod models;
mod prompts;

pub fn main() {
    let mut file_path = "piggy.json".to_string();

    find_configuration_file(&mut file_path);
    run(file_path);
}
