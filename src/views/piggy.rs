use crate::{commands::piggy_message::PiggyMessage, models::piggy::Piggy};
use iced::Sandbox;
use iced::widget::{column, row};

impl Sandbox for Piggy {
    type Message = PiggyMessage;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Piggy".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // let main_view = MainView::build_view(self);

        column![].push(row![]).into()
    }
}
