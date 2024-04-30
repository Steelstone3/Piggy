use crate::{commands::piggy_message::PiggyMessage, models::piggy_settings::PiggySettings};
use iced::widget::{column, text, text_input, Column, Text, TextInput};

pub struct FolderSelectionView<'a> {
    folder_selection_text: Text<'a>,
    folder_selection_input: TextInput<'a, PiggyMessage>,
}

impl FolderSelectionView<'_> {
    pub fn build_view<'a>(piggy_settings: &PiggySettings) -> Column<'a, PiggyMessage> {
        let folder_selection = FolderSelectionView::new(piggy_settings);

        column![
            folder_selection.folder_selection_text,
            folder_selection.folder_selection_input,
        ]
        // .spacing(10.0)
        // .padding(10.0)
    }

    fn new<'a>(piggy_settings: &PiggySettings) -> FolderSelectionView<'a> {
        FolderSelectionView {
            folder_selection_text: text("Project Folder:"),
            folder_selection_input: text_input("Enter Folder", &piggy_settings.folder)
                .on_input(PiggyMessage::FolderChanged),
        }
    }
}
