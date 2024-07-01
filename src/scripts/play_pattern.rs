use crossbeam_channel::{Receiver, RecvTimeoutError};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::ui::drum_machine_page::PlaybackState;

use super::play_audio::play_audio;

pub fn play_pattern(
    stream_handle: Arc<rodio::OutputStreamHandle>,
    playback_state: Arc<Mutex<PlaybackState>>,
    beat_pattern_receiver: Receiver<Vec<Vec<bool>>>,
    sequence_playing: Arc<AtomicBool>,
    selected_samples: BTreeMap<usize, String>,
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

        let playback_state = playback_state.lock().unwrap();
        let bpm = playback_state.bpm;
        let sequence_length = current_beat_pattern[0].len() as u32;
        drop(playback_state); // Release the lock as soon as possible

        let beat_start = Instant::now();

        println!(
            "Beat index: {}, Pattern length: {}",
            beat_index,
            current_beat_pattern.len()
        );
        for (file_index, file_pattern) in current_beat_pattern.iter().enumerate() {
            println!("Checking file index: {}", file_index);
            if beat_index < file_pattern.len() && file_pattern[beat_index] {
                println!("Beat active for file index: {}", file_index);
                if let Some(sample_name) = selected_samples.get(&file_index) {
                    println!("Playing sample: {}", sample_name);
                    play_audio(&stream_handle, sample_name.clone(), path);
                } else {
                    println!("No sample found for file index: {}", file_index);
                }
            }
        }

        beat_index = (beat_index + 1) % sequence_length as usize;

        let beat_duration = Duration::from_millis((60_000 / bpm) as u64) / beat_scale;
        let elapsed = beat_start.elapsed();
        if elapsed < beat_duration {
            let sleep_duration = beat_duration - elapsed;
            // Sleep in small intervals, checking for stop signal
            let sleep_start = Instant::now();
            while sleep_start.elapsed() < sleep_duration {
                if !sequence_playing.load(Ordering::SeqCst) {
                    return; // Exit immediately if playback is stopped
                }
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}
