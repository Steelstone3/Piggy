use crate::models::piggy_settings::PiggySettings;
use crate::{commands::piggy_message::PiggyMessage, models::piggy::Piggy};
use iced::widget::column;
use iced::Sandbox;

use super::piggy_view::PiggyView;

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
            PiggyMessage::FolderChanged(folder) => self.piggy_settings.update_folder(folder),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // let main_view = MainView::build_view(self);
        let piggy_view = PiggyView::build_view(self);

        column![].push(piggy_view).into()
    }
}
