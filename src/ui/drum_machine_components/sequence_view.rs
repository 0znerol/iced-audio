use iced::font::Style;
use iced::widget::{checkbox, Button};
use iced::{theme, Theme};
use iced::{
    widget::{Checkbox, Column, Row, Text},
    Background, Color, Length,
};

use crate::ui::drum_machine_page::{DrumMachine, Message};

impl DrumMachine {
    pub fn create_sequence_view(&self) -> Column<Message> {
        let record_button = Button::new(Text::new("Record")).on_press(Message::RecordPattern);

        self.selected_samples.iter().enumerate().fold(
            Column::new().push(record_button),
            |column, (file_index, file_name)| {
                let beat_row =
                    (0..self.sequence_state.sequence_length).fold(Row::new(), |row, beat_index| {
                        let checkbox = if beat_index == 0 || beat_index % 4 == 0 {
                            checkbox(
                                "",
                                self.sequence_state.beat_pattern[file_index][beat_index as usize],
                            )
                            .style(theme::Checkbox::Custom(Box::new(HighlightedCheckbox)))
                        } else {
                            checkbox(
                                "",
                                self.sequence_state.beat_pattern[file_index][beat_index as usize],
                            )
                        };

                        row.push(checkbox.on_toggle(move |checked| {
                            Message::UpdateBeatPattern(file_index, beat_index as usize, checked)
                        }))
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

struct HighlightedCheckbox;

impl checkbox::StyleSheet for HighlightedCheckbox {
    type Style = Theme;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let mut appearance = style.active(&theme::Checkbox::default(), is_checked);

        appearance.background = Some(Background::Color(if is_checked {
            Color::from_rgb(0.5, 0.1, 0.3) // Highlighted color when checked
        } else {
            Color::from_rgb(0.5, 0.1, 0.2) // Highlighted color when unchecked
        }))
        .unwrap();

        appearance
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let mut appearance = self.active(style, is_checked);
        if let Background::Color(color) = appearance.background {
            let new_color = Color {
                a: color.a,
                r: (color.r + 0.5).min(1.0),
                g: (color.g + 0.1).min(1.0),
                b: (color.b + 0.1).min(1.0),
            };
            appearance.background = Some(Background::Color(new_color)).unwrap();
        }
        appearance
    }
}
