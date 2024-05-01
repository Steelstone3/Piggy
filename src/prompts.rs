use std::num::ParseIntError;

use crate::models::job::Job;
use inquire::{Confirm, Text};

pub fn text_prompt(message: &str, help_prompt: &str, default_value: &str) -> String {
    Text::new(message)
        .with_help_message(help_prompt)
        .with_default(default_value)
        .prompt()
        .unwrap()
}

pub fn confirmation_yes(message: &str) -> bool {
    Confirm::new(message)
        .with_default(true)
        .prompt()
        .unwrap_or_default()
}

pub fn job_selection(options: &Vec<Job>) -> Option<&Job> {
    for (index, job) in options.iter().enumerate() {
        println!("{} > {}", index, job.name);
    }

    loop {
        let input = text_prompt("Select Job:", "Enter number of the job", "");

        if let Ok(index) = parse_usize_numeric_value(&input) {
            if let Some(job) = options.get(index) {
                return Some(job);
            } else {
                println!("Invalid job number. Please enter a valid number.");
            }
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}

fn parse_usize_numeric_value(input: &str) -> Result<usize, ParseIntError> {
    input.trim().parse::<usize>()
}
