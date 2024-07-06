use std::sync::{Arc, MutexGuard};

use iced::{
    futures::lock::Mutex,
    widget::{Checkbox, Column, Row, Text},
    Length,
};

use crate::ui::{
    synth::{Message, Synth},
    SequenceState,
};

impl Synth {
    pub fn create_synth_sequence(
        &self,
        sequence_state: &MutexGuard<'_, SequenceState>,
    ) -> Column<Message> {
        let note_pattern = &sequence_state.note_pattern;

        let sequence_view =
            self.notes
                .iter()
                .enumerate()
                .fold(Column::new(), |column, (note_index, note_name)| {
                    let beat_row =
                        (0..sequence_state.sequence_length).fold(Row::new(), |row, beat_index| {
                            row.push(
                                Checkbox::new("", note_pattern[note_index][beat_index as usize])
                                    .on_toggle(move |checked| {
                                        Message::ToggleNote(
                                            note_index,
                                            beat_index as usize,
                                            checked,
                                        )
                                    }),
                            )
                        });

                    column.push(
                        Row::new()
                            .push(Text::new(note_name).width(Length::Fixed(30.0)))
                            .push(beat_row),
                    )
                });
        return sequence_view;
    }
}
