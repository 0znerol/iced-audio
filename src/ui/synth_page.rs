use iced::{
    widget::{Button, Checkbox, Column, Row, Text},
    Command, Element, Length,
};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::SequenceState;

pub struct SynthPage {
    sequence_state: Arc<Mutex<SequenceState>>,
    notes: Vec<String>,
    is_playing: Arc<Mutex<bool>>,
    play_sender: mpsc::Sender<bool>,
}
// struct SequenceState {
//     sequence_length: u32,
//     note_pattern: Vec<Vec<bool>>,
// }

#[derive(Debug, Clone)]
pub enum Message {
    ToggleNote(usize, usize, bool),
    PlaySequence,
    StopSequence,
    PlaybackFinished,
}

impl SynthPage {
    pub fn new(sequence_state: Arc<Mutex<SequenceState>>) -> Self {
        let notes: Vec<_> = vec![
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        // let sequence_state = Arc::new(Mutex::new(SequenceState {
        //     sequence_length: main_sequence_state.lock().unwrap().sequence_length,
        //     beat_pattern: vec![
        //         vec![
        //             false;
        //             main_sequence_state.lock().unwrap().sequence_length as usize
        //         ];
        //         notes.clone().len()
        //     ],
        // }));

        let (play_sender, play_receiver) = mpsc::channel();

        let is_playing = Arc::new(Mutex::new(false));
        let is_playing_clone = is_playing.clone();
        let sequence_state_clone = sequence_state.clone();

        thread::spawn(move || {
            let mut stream_option: Option<(OutputStream, OutputStreamHandle)> = None;
            loop {
                if let Ok(should_play) = play_receiver.recv() {
                    if should_play {
                        *is_playing_clone.lock().unwrap() = true;
                        if stream_option.is_none() {
                            stream_option = OutputStream::try_default().ok();
                        }
                        if let Some((_, ref stream_handle)) = stream_option {
                            Self::play_sequence(
                                sequence_state_clone.clone(),
                                is_playing_clone.clone(),
                                stream_handle,
                            );
                        }
                    } else {
                        *is_playing_clone.lock().unwrap() = false;
                        stream_option = None;
                    }
                }
            }
        });

        SynthPage {
            sequence_state,
            notes,
            is_playing,
            play_sender,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ToggleNote(note_index, beat_index, checked) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                sequence_state.beat_pattern[note_index][beat_index] = checked;
                Command::none()
            }
            Message::PlaySequence => {
                let mut is_playing = self.is_playing.lock().unwrap();
                if !*is_playing {
                    *is_playing = true;
                    self.play_sender.send(true).unwrap();
                }
                Command::none()
            }
            Message::StopSequence => {
                let mut is_playing = self.is_playing.lock().unwrap();
                *is_playing = false;
                self.play_sender.send(false).unwrap();
                Command::none()
            }
            Message::PlaybackFinished => {
                let mut is_playing = self.is_playing.lock().unwrap();
                *is_playing = false;
                Command::none()
            }
        }
    }

    fn play_note(frequency: f32, duration: f32, stream_handle: &OutputStreamHandle) {
        let sink = Sink::try_new(stream_handle).unwrap();

        let source = rodio::source::SineWave::new(frequency)
            .take_duration(Duration::from_secs_f32(duration))
            .amplify(0.20);

        sink.append(source);
        sink.sleep_until_end();
    }

    fn play_sequence(
        sequence_state: Arc<Mutex<SequenceState>>,
        is_playing: Arc<Mutex<bool>>,
        stream_handle: &OutputStreamHandle,
    ) {
        while *is_playing.lock().unwrap() {
            let sequence_state = sequence_state.lock().unwrap();
            let note_pattern = sequence_state.beat_pattern.clone();
            let sequence_length = sequence_state.sequence_length;
            drop(sequence_state);

            for beat in 0..sequence_length {
                if !*is_playing.lock().unwrap() {
                    return;
                }
                for (note_index, note_row) in note_pattern.iter().enumerate() {
                    if note_row[beat as usize] {
                        let frequency = 440.0 * 2.0_f32.powf((note_index as f32 - 9.0) / 12.0);
                        Self::play_note(frequency, 0.25, stream_handle);
                    }
                }
                thread::sleep(Duration::from_millis(250));
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sequence_state = self.sequence_state.lock().unwrap();
        let note_pattern = &sequence_state.beat_pattern;

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

        let play_button = if *self.is_playing.lock().unwrap() {
            Button::new(Text::new("Stop")).on_press(Message::StopSequence)
        } else {
            Button::new(Text::new("Play")).on_press(Message::PlaySequence)
        };

        Column::new().push(sequence_view).push(play_button).into()
    }
}