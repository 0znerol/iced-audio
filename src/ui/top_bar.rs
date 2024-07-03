use super::{drum_machine_page, MainUi, Message, Page};
use iced::{
    theme,
    widget::{button, checkbox, slider, Button, Column, Row, Text},
    Length,
};

impl MainUi {
    pub fn create_top_bar(&self) -> Column<Message> {
        let state = self.sequence_state.lock().unwrap();
        let playback_state = self.drum_machine.playback_state.lock().unwrap();
        let sequence_length = state.sequence_length;
        let bpm = state.bpm;
        let play_sequence_on = playback_state.play_sequence_on.clone();
        let synth_play_sequence_on = self.synth_page.is_playing.lock().unwrap().clone();
        drop(state);
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
                button("Synth")
                    .on_press(Message::ChangePage(Page::Synth))
                    .style(if self.current_page == Page::Arranger {
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
                .push(Text::new(format!("Sequence Length: {}", sequence_length)))
                .push(slider(1..=64, sequence_length, |value| {
                    Message::UpdateSequenceLength(value)
                }))
                .push(Text::new(format!("BPM: {}", bpm)))
                .push(slider(60..=240, bpm, |value| Message::UpdateBpm(value)))
                .push(
                    checkbox("Play Both", play_sequence_on && synth_play_sequence_on)
                        .on_toggle(move |value| Message::StartBothSequences(value)),
                )
                .spacing(20),
        );

        top_bar
    }
}
