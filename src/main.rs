use iced::{Sandbox, Settings};
use models::piggy::Piggy;

mod commands;
mod models;
mod views;

pub fn main() -> iced::Result {
    Piggy::run(Settings::default())
}
