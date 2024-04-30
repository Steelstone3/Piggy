use iced::{window, Sandbox, Settings, Size};
use models::piggy::Piggy;

mod commands;
mod models;
mod views;

pub fn main() -> iced::Result {
    Piggy::run(Settings {
        window: window::Settings {
            min_size: Some(Size {
                width: 640.0,
                height: 480.0,
            }),
            ..Default::default()
        },
        ..Default::default()
    })
}
