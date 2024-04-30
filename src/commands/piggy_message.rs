#[derive(Debug, Clone)]
pub enum PiggyMessage {
    // Piggy Configuration
    ProjectFolderLocationChanged(String),
    PiggyConfigurationFileLocationChanged(String),
}
