use super::piggy_settings::PiggySettings;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Piggy {
    pub piggy_settings: PiggySettings,
}

impl Default for Piggy {
    fn default() -> Self {
        Self::new()
    }
}

impl Piggy {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
