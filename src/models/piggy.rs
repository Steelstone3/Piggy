use super::{job::Job, piggy_settings::PiggySettings};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Piggy {
    pub piggy_settings: PiggySettings,
    pub jobs: Vec<Job>,
}

impl Default for Piggy {
    fn default() -> Self {
        Self {
            piggy_settings: PiggySettings::default(),
            jobs: Job::create_default_jobs(),
        }
    }
}
