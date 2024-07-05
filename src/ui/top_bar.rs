use super::{drum_machine, MainUi, Message, Page};
use iced::{
    theme,
    widget::{button, checkbox, slider, Button, Column, Row, Text},
    Length, Renderer, Theme,
};

impl MainUi {
    pub fn create_top_bar(&self) -> Column<Message> {
        let state = self.sequence_state.lock().unwrap();
        let playback_state = self.drum_machine.playback_state.lock().unwrap();
        let sequence_length = state.sequence_length;
        let bpm = state.bpm;
        let drum_sequence_on = state.drum_sequence_on.clone();
        let synth_sequence_on = state.synth_sequence_on.clone();
        let synth_drum_sequence_on = self.synth.is_playing.lock().unwrap().clone();
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

        let is_checked = drum_sequence_on && synth_sequence_on;
        let play_button: iced::widget::Button<'_, Message, Theme, Renderer> =
            if drum_sequence_on && synth_drum_sequence_on {
                Button::new(Text::new("Stop Both")).on_press(Message::StartBothSequences(false))
            } else {
                Button::new(Text::new("Play Both")).on_press(Message::StartBothSequences(true))
            };
        let top_bar = Column::new().push(interface_buttons).push(
            Row::new()
                .push(Text::new(format!("Sequence Length: {}", sequence_length)))
                .push(slider(1..=64, sequence_length, |value| {
                    Message::UpdateSequenceLength(value)
                }))
                .push(Text::new(format!("BPM: {}", bpm)))
                .push(slider(60..=240, bpm, |value| Message::UpdateBpm(value)))
                .push(
                    play_button, // checkbox("Play Both", is_checked)
                                 //     .on_toggle(move |value| Message::StartBothSequences(value)),
                )
                .spacing(20),
        );

        top_bar
    }
}
