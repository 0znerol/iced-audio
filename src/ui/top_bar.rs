use iced::{
    theme,
    widget::{button, checkbox, slider, Button, Column, Row, Text},
    Length,
};

use super::main_ui::{AudioPlayer, Message, Page};

impl AudioPlayer {
    pub fn create_top_bar(&self) -> Column<Message> {
        let interface_buttons = Row::new()
            .push(
                button("Drum Machine")
                    .on_press(Message::ChangeInterface(Page::Sequencer))
                    .style(if self.current_interface == Page::Sequencer {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    }),
            )
            .push(
                button("Arranger")
                    .on_press(Message::ChangeInterface(Page::SampleBrowser))
                    .style(if self.current_interface == Page::SampleBrowser {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    }),
            )
            .push(
                button("Settings")
                    .on_press(Message::ChangeInterface(Page::Settings))
                    .style(if self.current_interface == Page::Settings {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    }),
            )
            .spacing(10);
        let top_bar = Column::new().push(interface_buttons).push(
            Row::new()
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
                ))
                .push(
                    checkbox("Play Sequence", self.sequence_state.play_sequence_on)
                        .on_toggle(Message::ToggleSequence),
                )
                .spacing(20),
        );

        top_bar
    }
}
