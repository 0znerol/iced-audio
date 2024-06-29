use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use iced::{
    widget::{checkbox, scrollable, slider, Button, Column, Container, Row, Text},
    Application, Command, Element, Length, Theme,
};
use rodio::OutputStream;

use crate::scripts::{
    get_audio_files::get_audio_files, play_audio::play_audio, play_sequence::play_sequence,
};

pub struct AudioPlayer {
    output_stream: OutputStream,
    stream_handle: Arc<rodio::OutputStreamHandle>,
    pub audio_files: Vec<String>,
    pub sequence_state: SequenceState,
    sequence_playing: Arc<AtomicBool>,
    pub selected_samples: BTreeMap<usize, String>,
}

pub struct SequenceState {
    pub play_sequence_on: bool,
    pub sequence_length: u32,
    pub beat_pattern: Vec<Vec<bool>>,
    pub bpm: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    // PlayAudio(String),
    ToggleSequence(bool),
    UpdateSequenceLength(u32),
    UpdateBeatPattern(usize, usize, bool),
    UpdateBPM(u32),
    PlayAndAddSample(String),
}

impl Application for AudioPlayer {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let audio_files = get_audio_files("drumKits/TR-808 Kit");
        let sequence_playing = Arc::new(AtomicBool::new(false));
        let selected_samples = BTreeMap::new();

        let sequence_state = SequenceState {
            play_sequence_on: false,
            sequence_length: 8,
            beat_pattern: vec![vec![false; 8]; selected_samples.len()],
            bpm: 120,
        };
        (
            AudioPlayer {
                output_stream: stream,
                stream_handle: Arc::new(stream_handle),
                audio_files,
                sequence_state,
                sequence_playing,
                selected_samples,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audio Player")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Message::PlayAudio(file_name) => {
            //     play_audio(&self.stream_handle, file_name);
            // }
            Message::ToggleSequence(on) => {
                self.sequence_state.play_sequence_on = on;
                self.sequence_playing.store(on, Ordering::SeqCst);
                if on {
                    let stream_handle = Arc::clone(&self.stream_handle);
                    let beat_pattern = self.sequence_state.beat_pattern.clone();
                    let audio_files = self.audio_files.clone();
                    let bpm = self.sequence_state.bpm;
                    let sequence_length = self.sequence_state.sequence_length;
                    let sequence_playing = Arc::clone(&self.sequence_playing);
                    let selected_samples = self.selected_samples.clone();

                    thread::spawn(move || {
                        play_sequence(
                            stream_handle,
                            beat_pattern,
                            audio_files,
                            bpm,
                            sequence_length,
                            sequence_playing,
                            selected_samples,
                        );
                    });
                }
            }
            Message::UpdateSequenceLength(length) => {
                self.sequence_state.sequence_length = length;
                for pattern in &mut self.sequence_state.beat_pattern {
                    pattern.resize(length as usize, false);
                }
            }
            Message::UpdateBeatPattern(file_index, beat_index, checked) => {
                if file_index < self.sequence_state.beat_pattern.len()
                    && beat_index < self.sequence_state.beat_pattern[file_index].len()
                {
                    self.sequence_state.beat_pattern[file_index][beat_index] = checked;
                }
            }
            Message::UpdateBPM(bpm) => {
                self.sequence_state.bpm = bpm;
            }
            Message::PlayAndAddSample(sample_name) => {
                if !self.selected_samples.values().any(|v| v == &sample_name) {
                    let new_index = self.selected_samples.len();
                    self.selected_samples.insert(new_index, sample_name.clone());
                    self.sequence_state.beat_pattern.push(vec![
                        false;
                        self.sequence_state.sequence_length
                            as usize
                    ]);
                    println!("beat patter :{:?}", self.sequence_state.beat_pattern);
                    println!("selected samples :{:?}", self.selected_samples);
                }
                play_audio(&self.stream_handle, sample_name.clone());
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let top_bar = self.create_top_bar();
        let sequence_view = self.create_sequence_view();
        let sample_buttons = self.create_sample_buttons();

        let all_content = Column::new()
            .push(top_bar)
            .push(sequence_view)
            .push(Text::new("Sample Buttons").size(20))
            .push(sample_buttons)
            .spacing(20);

        scrollable(Container::new(all_content).width(Length::Fill).padding(20))
            .height(Length::Fill)
            .into()
    }
}
