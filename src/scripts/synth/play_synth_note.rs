use std::time::Duration;

use rodio::{OutputStreamHandle, Sink, Source};

use crate::ui::synth::Synth;

impl Synth {
    pub fn play_note(frequency: f32, duration: Duration, stream_handle: &OutputStreamHandle) {
        let sink = Sink::try_new(stream_handle).unwrap();

        let source = rodio::source::SineWave::new(frequency)
            .take_duration(duration)
            .amplify(0.20);

        sink.append(source);
        sink.sleep_until_end();
    }
}
