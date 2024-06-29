use iced::widget::{checkbox, slider, Column, Text};

use super::midi_player::{AudioPlayer, Message};

impl AudioPlayer {
    pub fn create_top_bar(&self) -> Column<Message> {
        let top_bar = Column::new()
            .push(Text::new("Audio Player"))
            .push(
                checkbox("Play Sequence", self.sequence_state.play_sequence_on)
                    .on_toggle(Message::ToggleSequence),
            )
            .push(Text::new(format!(
                "Sequence Length: {}",
                self.sequence_state.sequence_length
            )))
            .push(slider(
                1..=64,
                self.sequence_state.sequence_length,
                |value| Message::UpdateSequenceLength(value),
            ))
            .push(Text::new(format!("BPM: {}", self.sequence_state.bpm)))
            .push(slider(
                60..=240,
                self.sequence_state.bpm,
                Message::UpdateBPM,
            ));
        top_bar
    }
}
