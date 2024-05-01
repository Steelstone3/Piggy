use prompts::{confirmation, text_prompt};

mod controllers;
mod models;
mod prompts;

pub fn main() {
    if confirmation("Use default piggy.toml file?") {
        // TODO detect piggy.toml exists in default location
        if confirmation("Default piggy.toml file was not detected. Would you like to create one?") {
            // TODO create a default piggy.toml file
        }
    } else {
        // TODO detect piggy.toml exists in specified location
        text_prompt(
            "Specify piggy.toml file path",
            "Where is piggy.toml on your system?",
            ".",
        );
    }

    
}
