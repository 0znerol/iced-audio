use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rodio::OutputStreamHandle;

use crate::ui::{drum_machine::SequenceScale, synth::Synth, SequenceState};

impl Synth {
    pub fn play_sequence(
        sequence_state: Arc<Mutex<SequenceState>>,
        is_playing: Arc<Mutex<bool>>,
        stream_handle: &OutputStreamHandle,
    ) {
        while *is_playing.lock().unwrap() {
            let sequence_state = sequence_state.lock().unwrap();
            let note_pattern = sequence_state.note_pattern.clone();
            let sequence_length = sequence_state.sequence_length;
            let bpm = sequence_state.bpm;
            let octave = sequence_state.octave;
            let frequency = sequence_state.frequency;
            let sequence_scale = match sequence_state.synth_scale {
                SequenceScale::OneFourth => 1,
                SequenceScale::OneEighth => 2,
                SequenceScale::OneSixteenth => 4,
            };
            drop(sequence_state);

            let beat_duration = Duration::from_millis((60_000 / bpm) as u64);
            let note_duration = beat_duration / sequence_scale;

            for beat in 0..sequence_length {
                if !*is_playing.lock().unwrap() {
                    return;
                }
                for (note_index, note_row) in note_pattern.iter().enumerate() {
                    if note_row[beat as usize] {
                        let base_frequency = frequency * 2.0_f32.powf((octave) as f32);
                        let frequency =
                            base_frequency * 2.0_f32.powf((note_index as f32 - 9.0) / 12.0);
                        let stream_handle = stream_handle.clone();
                        thread::spawn(move || {
                            Self::play_note(frequency, note_duration, &stream_handle);
                        });
                    }
                }
                thread::sleep(note_duration);
            }
        }
    }
}
