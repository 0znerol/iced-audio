use iced::alignment::{self, Vertical};
use iced::font::Style;
use iced::futures::future::select;
use iced::widget::{button, checkbox, container, Button, PickList};
use iced::{theme, Renderer, Theme};
use iced::{
    widget::{Checkbox, Column, Row, Text},
    Background, Color, Length,
};

use crate::ui::drum_machine_page::{self, DrumMachine, Message, SequenceScale};

impl DrumMachine {
    pub fn create_sequence_view(&self) -> Column<Message> {
        let record_button = Button::new(Text::new("Record")).on_press(Message::RecordPattern);
        let sequence_length_pick_list: iced::widget::PickList<
            '_,
            SequenceScale,
            Vec<SequenceScale>,
            SequenceScale,
            drum_machine_page::Message,
            Theme,
            Renderer,
        > = PickList::new(
            self.sequence_scale_options.clone(),
            Some(self.sequence_scale),
            Message::ChangeSequenceScale,
        );
        // let current_beat_index = *self.current_beat_index.lock().unwrap();
        self.selected_samples.iter().enumerate().fold(
            Column::new()
                .push(record_button)
                .push(sequence_length_pick_list),
            |column, (file_index, file_name)| {
                let sequence_state = self.sequence_state.lock().unwrap();
                let beat_pattern = sequence_state.beat_pattern.clone();
                let sequence_length = sequence_state.sequence_length;
                drop(sequence_state);

                // Add this check
                if beat_pattern.is_empty() || file_index >= beat_pattern.len() {
                    return column; // Skip this iteration if beat_pattern is empty or index is out of bounds
                }
                let beat_row = (0..sequence_length).fold(Row::new(), |row, beat_index| {
                    let checkbox = if beat_index == 0 || beat_index % 4 == 0
                    // || current_beat_index == beat_index as usize
                    {
                        checkbox("", beat_pattern[file_index][beat_index as usize])
                            .style(theme::Checkbox::Custom(Box::new(HighlightedCheckbox)))
                    } else {
                        checkbox("", beat_pattern[file_index][beat_index as usize])
                    };

                    row.push(checkbox.on_toggle(move |checked| {
                        Message::UpdateBeatPattern(file_index, beat_index as usize, checked)
                    }))
                });
                let remove_button = button(
                    Text::new("X")
                        .size(20)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .on_press(Message::RemoveSample(file_index))
                .padding(0)
                .width(Length::Fixed(30.0))
                .height(Length::Fixed(30.0))
                .style(theme::Button::Text);

                let remove_button_container = container(remove_button)
                    .width(Length::Fixed(30.0))
                    .height(Length::Fixed(30.0))
                    .center_y();

                column.push(
                    Row::new()
                        .push(
                            Text::new(file_name.1.keys().next().unwrap())
                                .size(15)
                                .width(Length::Fixed(150.0))
                                .height(Length::Fixed(20.0)),
                        )
                        .push(beat_row)
                        .push(remove_button_container)
                        .align_items(alignment::Alignment::Center),
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
