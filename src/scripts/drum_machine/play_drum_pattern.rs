use std::{
    collections::{BTreeMap, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use crossbeam_channel::{select, Receiver};
use rodio::OutputStreamHandle;

use crate::ui::{
    drum_machine::{DrumMachine, SampleFolder, SequenceScale},
    SequenceState,
};

impl DrumMachine {
    pub fn play_pattern(
        sequence_state: Arc<Mutex<SequenceState>>,
        is_playing: Arc<Mutex<bool>>,
        stream_handle: &OutputStreamHandle,
        selected_samples: Arc<RwLock<BTreeMap<usize, HashMap<String, SampleFolder>>>>,
        root_sample_folder: &str,
    ) {
        while *is_playing.lock().unwrap() {
            let sequence_state = sequence_state.lock().unwrap();
            let beat_pattern = sequence_state.beat_pattern.clone();
            let sequence_length = sequence_state.sequence_length;
            let bpm = sequence_state.bpm;

            let drum_scale = sequence_state.drum_scale;
            drop(sequence_state);
            let scale = match drum_scale {
                SequenceScale::OneFourth => 1,
                SequenceScale::OneEighth => 2,
                SequenceScale::OneSixteenth => 4,
                _ => 1,
            };
            let beat_duration = Duration::from_millis((60_000 / bpm) as u64) / scale;

            for beat in 0..sequence_length {
                if !*is_playing.lock().unwrap() {
                    return;
                }
                let selected_samples = selected_samples.read().unwrap();
                for (file_index, file_pattern) in beat_pattern.iter().enumerate() {
                    if file_pattern[beat as usize] {
                        if let Some(sample_map) = selected_samples.get(&file_index) {
                            let sample_folder = sample_map.values().next().unwrap().to_string();
                            let full_path = root_sample_folder.to_string() + "/" + &sample_folder;
                            let sample_name = sample_map.keys().next().unwrap().clone();
                            let stream_handle = Arc::new(stream_handle.clone());

                            thread::spawn(move || {
                                Self::play_audio(
                                    &stream_handle,
                                    beat_duration,
                                    sample_name,
                                    &full_path,
                                );
                            });
                        }
                    }
                }
                thread::sleep(beat_duration);
            }
        }
    }
}
