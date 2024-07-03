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

use crate::{
    scripts::play_audio::play_audio,
    ui::{drum_machine_page::SampleFolder, SequenceState},
};

pub fn play_pattern(
    stream_handle: Arc<rodio::OutputStreamHandle>,
    sequence_state: Arc<Mutex<SequenceState>>,
    beat_pattern_receiver: Receiver<Vec<Vec<bool>>>,
    sequence_playing: Arc<AtomicBool>,
    selected_samples: Arc<RwLock<BTreeMap<usize, HashMap<String, SampleFolder>>>>,
    path: &str,
    beat_scale: u32,
    initial_beat_pattern: Vec<Vec<bool>>,
) {
    let mut current_beat_pattern = initial_beat_pattern;
    let mut beat_index = 0;
    let start_time = Instant::now();

    while sequence_playing.load(Ordering::SeqCst) {
        let initial_state = sequence_state.lock().unwrap();
        let bpm = initial_state.bpm;
        let sequence_length = current_beat_pattern[0].len() as u32;
        drop(initial_state);

        let beat_duration = Duration::from_millis((60_000 / bpm) as u64) / beat_scale;
        let note_duration = beat_duration / 4;

        // Use select! to check for both time to play and pattern updates
        select! {
            recv(beat_pattern_receiver) -> msg => {
                if let Ok(new_pattern) = msg {
                    current_beat_pattern = new_pattern;
                    println!("Updated beat pattern");
                }
            }
            default(beat_duration) => {
                println!("Playing audio for beat {}", beat_index);
                // Play the audio for this beat
                let selected_samples = selected_samples.read().unwrap();
                for (file_index, file_pattern) in current_beat_pattern.iter().enumerate() {
                    if beat_index < file_pattern.len() && file_pattern[beat_index] {
                        if let Some(sample_map) = selected_samples.get(&file_index) {
                            let sample_folder = sample_map.values().next().unwrap().to_string();
                            let full_path = path.to_string() + &sample_folder;
                            let sample_name = sample_map.keys().next().unwrap().clone();
                            let stream_handle = Arc::clone(&stream_handle);

                            println!("Playing sample: {:?}", sample_name);
                            thread::spawn(move || {
                                play_audio(&stream_handle, note_duration, sample_name, &full_path);
                            });
                        }
                    }
                }

                if beat_index < sequence_length as usize - 1 {
                    beat_index += 1;
                } else {
                    beat_index = 0;
                }
            }
        }
    }
    println!("Playback loop ended");
}
