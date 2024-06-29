use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use tokio::time::Instant;

use super::play_audio::play_audio;

pub fn play_sequence(
    stream_handle: Arc<rodio::OutputStreamHandle>,
    beat_pattern: Vec<Vec<bool>>,
    audio_files: Vec<String>,
    bpm: u32,
    sequence_length: u32,
    sequence_playing: Arc<AtomicBool>,
    selected_samples: BTreeMap<usize, String>,
) {
    let beat_duration = Duration::from_millis((60_000 / bpm) as u64);

    while sequence_playing.load(Ordering::SeqCst) {
        let start_time = Instant::now();

        for beat in 0..sequence_length {
            for (file_index, file_pattern) in beat_pattern.iter().enumerate() {
                if file_index < file_pattern.len()
                    && beat < file_pattern.len() as u32
                    && file_pattern[beat as usize]
                {
                    play_audio(
                        &stream_handle,
                        selected_samples.get(&file_index).unwrap().clone(),
                    );
                }
            }

            let elapsed = start_time.elapsed();
            let target_time = beat_duration * (beat + 1);
            if elapsed < target_time {
                thread::sleep(target_time - elapsed);
            }
        }
    }
}
