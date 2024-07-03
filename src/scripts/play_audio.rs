use std::{fs::File, io::BufReader, path::Path, sync::Arc, thread, time::Duration};

use rodio::{Decoder, Sink, Source};

pub fn play_audio(
    stream_handle: &Arc<rodio::OutputStreamHandle>,
    note_duration: Duration,
    file_name: String,
    path: &str,
) {
    let path = std::path::Path::new(path).join(&file_name);
    let file = std::fs::File::open(path).unwrap();
    let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();

    let sink = Sink::try_new(stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end()
    // Use a separate thread to stop the sink after the note duration
    // let sink_handle = sink.clone();
    // std::thread::spawn(move || {
    //     std::thread::sleep(note_duration);
    //     sink.stop();
    // });
}
