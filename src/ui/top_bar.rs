use super::{drum_machine_page, MainUi, Message, Page};
use iced::{
    theme,
    widget::{button, checkbox, slider, Button, Column, Row, Text},
    Length,
};

impl MainUi {
    pub fn create_top_bar(&self) -> Column<Message> {
        let interface_buttons = Row::new()
            .push(
                button("Drum Machine")
                    .on_press(Message::ChangePage(Page::DrumMachine))
                    .style(if self.current_page == Page::DrumMachine {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    }),
            )
            .push(
                button("Arranger")
                    .on_press(Message::ChangePage(Page::Arranger))
                    .style(if self.current_page == Page::Arranger {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    }),
            )
            .push(
                button("Settings")
                    .on_press(Message::ChangePage(Page::Settings))
                    .style(if self.current_page == Page::Settings {
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
                    self.drum_machine.sequence_state.sequence_length
                )))
                .push(slider(
                    1..=64,
                    self.drum_machine.sequence_state.sequence_length,
                    |value| {
                        Message::DrumMachineMessage(
                            drum_machine_page::Message::UpdateSequenceLength(value),
                        )
                    },
                ))
                .push(Text::new(format!(
                    "BPM: {}",
                    self.drum_machine.sequence_state.bpm
                )))
                .push(slider(
                    60..=240,
                    self.drum_machine.sequence_state.bpm,
                    |value| {
                        Message::DrumMachineMessage(drum_machine_page::Message::UpdateBPM(value))
                    },
                ))
                .push(
                    checkbox(
                        "Play Sequence",
                        self.drum_machine.sequence_state.play_sequence_on,
                    )
                    .on_toggle(|value| {
                        Message::DrumMachineMessage(drum_machine_page::Message::ToggleSequence(
                            value,
                        ))
                    }),
                )
                .spacing(20),
        );

        top_bar
    }
}
