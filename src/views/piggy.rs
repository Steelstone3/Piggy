use super::piggy_view::PiggyView;
use crate::models::piggy_settings::PiggySettings;
use crate::{commands::piggy_message::PiggyMessage, models::piggy::Piggy};
use iced::widget::column;
use iced::Sandbox;

impl Sandbox for Piggy {
    type Message = PiggyMessage;

    fn new() -> Self {
        Self {
            piggy_settings: PiggySettings::default(),
        }
    }

    fn title(&self) -> String {
        "Piggy".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            PiggyMessage::ProjectFolderLocationChanged(project_folder_location) => {
                self.piggy_settings.update_folder(project_folder_location)
            }
            PiggyMessage::PiggyConfigurationFileLocationChanged(
                piggy_configuration_file_location,
            ) => self
                .piggy_settings
                .update_configuration_file(piggy_configuration_file_location),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let piggy_view = PiggyView::build_view(self);

        column![].push(piggy_view).into()
    }
}
