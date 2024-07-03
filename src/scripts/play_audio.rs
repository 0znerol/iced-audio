use std::{fs::File, io::BufReader, path::Path, sync::Arc, thread, time::Duration};

use rodio::{Decoder, Sink, Source};

pub fn play_audio(
    stream_handle: &Arc<rodio::OutputStreamHandle>,
    note_duration: Duration,
    file_name: String,
    path: &str,
) {
    let path = Path::new(path).join(&file_name);
    // Spawn a new thread for audio playback
    let output_handle = stream_handle.clone();
    thread::spawn(move || {
        // Create a new sink for each playback
        let sink = Sink::try_new(&output_handle).unwrap();
        // Load and play the audio file
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        // Wait for the sound to finish playing
        sink.sleep_until_end();
    });
}
