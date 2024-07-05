use iced::{
    widget::{checkbox, Button, Column, PickList, Row, Text},
    Length, Renderer, Theme,
};

use crate::ui::drum_machine::{self, DrumMachine, Message, SampleFolder};

impl DrumMachine {
    pub fn create_sample_buttons(&self) -> Column<Message> {
        let mut sorted_files = self.audio_files.clone();
        sorted_files.sort();
        let folder_pick_list: iced::widget::PickList<
            '_,
            SampleFolder,
            Vec<SampleFolder>,
            SampleFolder,
            Message,
            Theme,
            Renderer,
        > = PickList::new(
            self.sample_folders_options.clone(),
            Some(self.sample_folder.clone()),
            Message::ChangeSampleFolder,
        );
        let add_sample_checkbox: iced::widget::Checkbox<'_, Message, Theme, Renderer> = checkbox(
            "Add sample to pattern",
            self.add_sample_on_play,
            // Message::ToggleAddSampleOnPlay,
        )
        .on_toggle(Message::ToggleAddSampleOnPlay);

        let column: iced::widget::Column<'_, Message, Theme, Renderer> = Column::new()
            .push(Text::new("Sample Buttons").size(20))
            .push(
                Row::new()
                    .push(folder_pick_list)
                    .push(add_sample_checkbox)
                    .spacing(10),
            )
            .spacing(10);

        sorted_files.chunks(4).fold(column, |column, chunk| {
            let row = chunk.iter().fold(Row::new().spacing(10), |row, file_name| {
                let file_name = file_name.clone();
                row.push(
                    Button::new(Text::new(file_name.clone()))
                        .on_press(Message::PlayAndAddSample(file_name))
                        .padding(5)
                        .width(Length::FillPortion(1))
                        .height(Length::Fixed(80.0)),
                )
            });
            column.push(row)
        })
    }
}
