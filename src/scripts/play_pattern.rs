use crossbeam_channel::{Receiver, RecvTimeoutError};
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::ui::drum_machine_page::{PlaybackState, SampleFolder};
use crate::ui::SequenceState;

use super::play_audio::play_audio;

pub fn play_pattern(
    stream_handle: Arc<rodio::OutputStreamHandle>,
    sequence_state: Arc<Mutex<SequenceState>>,
    beat_pattern_receiver: Receiver<Vec<Vec<bool>>>,
    sequence_playing: Arc<AtomicBool>,
    selected_samples: BTreeMap<usize, HashMap<String, SampleFolder>>,
    path: &str,
    beat_scale: u32,
    initial_beat_pattern: Vec<Vec<bool>>,
) {
    let mut current_beat_pattern = initial_beat_pattern;
    let mut beat_index = 0;

    while sequence_playing.load(Ordering::SeqCst) {
        // let start_time = Instant::now();

        // Check for beat pattern updates with a timeout
        match beat_pattern_receiver.recv_timeout(Duration::from_millis(10)) {
            Ok(new_pattern) => current_beat_pattern = new_pattern,
            Err(RecvTimeoutError::Timeout) => {} // No new pattern, continue with current
            Err(RecvTimeoutError::Disconnected) => break, // Channel closed, exit loop
        }

        if current_beat_pattern.is_empty() {
            println!("No pattern to play");

            continue; // No pattern to play
        }

        let state = sequence_state.lock().unwrap();
        let bpm = state.bpm;
        let sequence_length = current_beat_pattern[0].len() as u32;
        drop(state); // Release the lock as soon as possible

        let beat_start = Instant::now();
        let beat_duration = Duration::from_millis((60_000 / bpm) as u64) / beat_scale;
        let note_duration = beat_duration / 4; // Assuming quarter notes

        println!(
            "Beat index: {}, Pattern length: {}",
            beat_index,
            current_beat_pattern.len()
        );
        for (file_index, file_pattern) in current_beat_pattern.iter().enumerate() {
            println!("Checking file index: {}", file_index);
            if beat_index < file_pattern.len() && file_pattern[beat_index] {
                println!("Beat active for file index: {}", file_index);
                if let Some(sample_map) = selected_samples.get(&file_index) {
                    println!("Playing sample: {}", sample_map.keys().next().unwrap());
                    let sample_folder = sample_map.values().next().unwrap().to_string();
                    let full_path = path.to_string() + &sample_folder;
                    play_audio(
                        &stream_handle,
                        note_duration,
                        sample_map.keys().next().unwrap().clone(),
                        &full_path,
                    );
                } else {
                    println!("No sample found for file index: {}", file_index);
                }
            }
        }

        beat_index = (beat_index + 1) % sequence_length as usize;

        let elapsed = beat_start.elapsed();
        if elapsed < beat_duration {
            let sleep_duration = beat_duration - elapsed;
            // Sleep in small intervals, checking for stop signal
            let sleep_start = Instant::now();
            while sleep_start.elapsed() < sleep_duration {
                if !sequence_playing.load(Ordering::SeqCst) {
                    return; // Exit immediately if playback is stopped
                }
                thread::sleep(Duration::from_millis(0));
            }
        }
    }
}
