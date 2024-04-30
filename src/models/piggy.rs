use iced::Sandbox;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Piggy {}

impl Default for Piggy {
    fn default() -> Self {
        Self::new()
    }
}

// TODO controller wrapper functions?
impl Piggy {}
