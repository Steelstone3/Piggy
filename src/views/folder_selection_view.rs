use crate::{commands::piggy_message::PiggyMessage, models::piggy_settings::PiggySettings};
use iced::widget::{column, text, text_input, Column, Text, TextInput};

pub struct ConfigurationView<'a> {
    project_folder_location_text: Text<'a>,
    project_folder_location_input: TextInput<'a, PiggyMessage>,
    piggy_configuration_file_location_text: Text<'a>,
    piggy_configuration_file_location_input: TextInput<'a, PiggyMessage>,
}

impl ConfigurationView<'_> {
    pub fn build_view<'a>(piggy_settings: &PiggySettings) -> Column<'a, PiggyMessage> {
        let configuration = ConfigurationView::new(piggy_settings);

        column![
            configuration.project_folder_location_text,
            configuration.project_folder_location_input,
            configuration.piggy_configuration_file_location_text,
            configuration.piggy_configuration_file_location_input,
        ]
        .spacing(10.0)
        .padding(10.0)
    }

    fn new<'a>(piggy_settings: &PiggySettings) -> ConfigurationView<'a> {
        ConfigurationView {
            project_folder_location_text: text("Project Folder:"),
            project_folder_location_input: text_input(
                "Enter Folder Location",
                &piggy_settings.folder,
            )
            .on_input(PiggyMessage::ProjectFolderLocationChanged),
            piggy_configuration_file_location_text: text("Configuration File:"),
            piggy_configuration_file_location_input: text_input(
                "Enter Piggy Configuration File Location",
                &piggy_settings.configuration_file,
            )
            .on_input(PiggyMessage::PiggyConfigurationFileLocationChanged),
        }
    }
}
