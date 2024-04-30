use super::folder_selection_view::ConfigurationView;
use crate::{commands::piggy_message::PiggyMessage, models::piggy::Piggy};
use iced::widget::{column, Column};

pub struct PiggyView<'a> {
    folder_selection: Column<'a, PiggyMessage>,
}

impl PiggyView<'_> {
    pub fn build_view<'a>(piggy: &Piggy) -> Column<'a, PiggyMessage> {
        let folder_selection_view = PiggyView::new(piggy);

        column![].push(folder_selection_view.folder_selection)
    }

    fn new<'a>(piggy: &Piggy) -> PiggyView<'a> {
        PiggyView {
            folder_selection: ConfigurationView::build_view(&piggy.piggy_settings),
        }
    }
}
