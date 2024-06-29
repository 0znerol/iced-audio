use iced::{
    widget::{checkbox, Column, Row, Text},
    Length,
};

use super::midi_player::{AudioPlayer, Message};

impl AudioPlayer {
    pub fn create_sequence_view(&self) -> Column<Message> {
        self.selected_samples.iter().enumerate().fold(
            Column::new(),
            |column, (file_index, file_name)| {
                let beat_row =
                    (0..self.sequence_state.sequence_length).fold(Row::new(), |row, beat_index| {
                        row.push(
                            checkbox(
                                "",
                                self.sequence_state.beat_pattern[file_index][beat_index as usize],
                            )
                            .on_toggle(move |checked| {
                                Message::UpdateBeatPattern(file_index, beat_index as usize, checked)
                            }),
                        )
                    });

                column.push(
                    Row::new()
                        .push(
                            Text::new(file_name.1)
                                .size(15)
                                .width(Length::Fixed(100.0))
                                .height(Length::Fixed(70.0)),
                        )
                        .push(beat_row),
                )
            },
        )
    }
}
