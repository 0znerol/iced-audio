use iced::{
    widget::{checkbox, Button, Column, PickList, Row, Text},
    Length, Renderer, Theme,
};

use crate::ui::drum_machine_page::{self, DrumMachine, Message, SampleFolder};

impl DrumMachine {
    pub fn create_sample_buttons(&self) -> Column<Message> {
        let mut sorted_files = self.audio_files.clone();
        sorted_files.sort();

        sorted_files
            .chunks(4)
            .fold(Column::new().spacing(10), |column, chunk| {
                let row = chunk.iter().fold(Row::new().spacing(10), |row, file_name| {
                    let file_name = file_name.clone(); // Clone the file name here
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
