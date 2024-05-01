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
    for job in options {
        println!("> {}", job.name)
    }

    let input = text_prompt("Select Job:", "Enter number of the job", "");

    options.get(parse_usize_numeric_value(input))
}

fn parse_usize_numeric_value(input: String) -> usize {
    match input.chars().find(|character| character.is_numeric()) {
        Some(_) => input.as_str().trim().parse::<usize>().unwrap(),
        None => panic!("Not a numeric value"),
    }
}
