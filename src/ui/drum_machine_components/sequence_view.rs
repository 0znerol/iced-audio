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

        let mut column = Column::new().push(
            Row::new()
                .spacing(10)
                .push(record_button)
                .push(Text::new("Scale:").size(20))
                .push(sequence_length_pick_list),
        );

        let selected_samples = self.selected_samples.read().unwrap();
        let sequence_state = self.sequence_state.lock().unwrap();
        let beat_pattern = sequence_state.beat_pattern.clone();
        let sequence_length = sequence_state.sequence_length;
        drop(sequence_state);

        for (file_index, (_, file_map)) in selected_samples.iter().enumerate() {
            if beat_pattern.is_empty() || file_index >= beat_pattern.len() {
                continue; // Skip this iteration if beat_pattern is empty or index is out of bounds
            }

            let sample_name = file_map.keys().next().unwrap().clone();
            let beat_row = (0..sequence_length).fold(Row::new(), |row, beat_index| {
                let is_active = beat_pattern[file_index][beat_index as usize];
                let checkbox = if beat_index == 0 || beat_index % 4 == 0 {
                    checkbox("", is_active)
                        .style(theme::Checkbox::Custom(Box::new(HighlightedCheckbox)))
                } else {
                    checkbox("", is_active)
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

            column = column.push(
                Row::new()
                    .push(
                        Text::new(sample_name)
                            .size(15)
                            .width(Length::Fixed(150.0))
                            .height(Length::Fixed(20.0)),
                    )
                    .push(beat_row)
                    .push(remove_button_container)
                    .align_items(alignment::Alignment::Center),
            );
        }

        column
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
