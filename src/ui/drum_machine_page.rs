use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, RwLock,
    },
    thread,
};

use iced::{
    widget::{checkbox, scrollable, slider, Button, Column, Container, PickList, Row, Text},
    Command, Element, Length, Renderer, Theme,
};
use rodio::OutputStream;

use crate::scripts::{
    get_audio_files::get_audio_files, play_audio::play_audio, play_pattern::play_pattern,
    record_pattern::record_pattern,
};

use super::{MainUi, Page};

pub struct DrumMachine {
    output_stream: OutputStream,
    stream_handle: Arc<rodio::OutputStreamHandle>,
    pub audio_files: Vec<String>,
    pub sequence_state: Arc<Mutex<SequenceState>>,
    sequence_playing: Arc<AtomicBool>,
    pub selected_samples: BTreeMap<usize, HashMap<String, SampleFolder>>,
    pub sequence_scale_options: Vec<SequenceScale>,
    pub sequence_scale: SequenceScale,
    pub playback_state: Arc<Mutex<PlaybackState>>,
    pub beat_pattern_sender: crossbeam_channel::Sender<Vec<Vec<bool>>>,
    pub beat_pattern_receiver: crossbeam_channel::Receiver<Vec<Vec<bool>>>,
    pub root_sample_folder: String,
    pub sample_folders_options: Vec<SampleFolder>,
    pub sample_folder: SampleFolder,
}

#[derive(Debug, Clone, PartialEq)]

pub enum SampleFolder {
    NineONine,
    EightOEight,
}
impl fmt::Display for SampleFolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SampleFolder::NineONine => write!(f, "909"),
            SampleFolder::EightOEight => write!(f, "TR-808 Kit"),
        }
    }
}

pub struct SequenceState {
    pub sequence_length: u32,
    pub beat_pattern: Vec<Vec<bool>>,
}

pub struct PlaybackState {
    pub play_sequence_on: bool,
    pub bpm: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleDrumSequence(bool),
    UpdateSequenceLength(u32),
    UpdateBeatPattern(usize, usize, bool),
    UpdateBPM(u32),
    PlayAndAddSample(String),
    RecordPattern,
    ChangeSequenceScale(SequenceScale),
    RemoveSample(usize),
    ChangeSampleFolder(SampleFolder),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceScale {
    OneFourth,
    OneEighth,
    OneSixteenth,
}
impl fmt::Display for SequenceScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SequenceScale::OneFourth => write!(f, "1/4"),
            SequenceScale::OneEighth => write!(f, "1/8"),
            SequenceScale::OneSixteenth => write!(f, "1/16"),
        }
    }
}

impl DrumMachine {
    pub fn new() -> (Self, Command<Message>) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let audio_files = get_audio_files("drumKits/909");
        let sequence_playing = Arc::new(AtomicBool::new(false));
        let selected_samples = BTreeMap::new();
        let sequence_scale_options = vec![
            SequenceScale::OneFourth,
            SequenceScale::OneEighth,
            SequenceScale::OneSixteenth,
        ];
        let sequence_scale = SequenceScale::OneFourth;

        let (beat_pattern_sender, beat_pattern_receiver) = crossbeam_channel::unbounded();

        let sequence_state = Arc::new(Mutex::new(SequenceState {
            sequence_length: 16,
            beat_pattern: vec![vec![false; 16]; 0],
        }));

        let playback_state = Arc::new(Mutex::new(PlaybackState {
            play_sequence_on: false,
            bpm: 120,
        }));
        let root_sample_folder = "drumKits".to_string();
        let sample_folders_options = vec![SampleFolder::NineONine, SampleFolder::EightOEight];
        let sample_folder = SampleFolder::NineONine;
        (
            DrumMachine {
                output_stream: stream,
                stream_handle: Arc::new(stream_handle),
                audio_files,
                sequence_state,
                playback_state,
                beat_pattern_sender,
                beat_pattern_receiver,
                sequence_playing,
                selected_samples,
                sequence_scale_options,
                sequence_scale,
                root_sample_folder,
                sample_folders_options,
                sample_folder,
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangeSampleFolder(folder) => {
                self.sample_folder = folder.clone();
                self.audio_files =
                    get_audio_files(&format!("{}/{}", self.root_sample_folder, folder));
            }
            Message::RemoveSample(index) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                let mut beat_pattern = &mut sequence_state.beat_pattern.clone();
                drop(sequence_state);

                if self.selected_samples.contains_key(&index) {
                    self.selected_samples.remove(&index);
                    beat_pattern.remove(index);

                    // let file_map = HashMap::new();
                    // file_map.insert(sample_name, self.sample_folder.clone());

                    // self.selected_samples.insert(new_index, file_map);
                    // Reindex the remaining samples
                    let new_samples: BTreeMap<usize, HashMap<String, SampleFolder>> = self
                        .selected_samples
                        .values() // get the values, which are the HashMaps
                        .cloned() // clone each HashMap
                        .enumerate() // enumerate the cloned HashMaps
                        .collect();
                    self.selected_samples = new_samples;
                }
            }
            Message::ChangeSequenceScale(new_sequence_size) => {
                self.sequence_scale = new_sequence_size;
                return Command::none();
            }
            Message::RecordPattern => {
                let sequence_state = self.sequence_state.lock().unwrap();
                let playback_state = self.playback_state.lock().unwrap();
                let path = self.root_sample_folder.clone() + "/";
                let beat_scale = match self.sequence_scale {
                    SequenceScale::OneEighth => 2,
                    SequenceScale::OneSixteenth => 4,
                    SequenceScale::OneFourth => 1,
                };
                let output_file = format!(
                    "pattern_{}.wav",
                    chrono::Local::now().format("%Y%m%d_%H%M%S")
                );
                if let Err(e) = record_pattern(
                    &sequence_state.beat_pattern,
                    &self.audio_files,
                    playback_state.bpm,
                    sequence_state.sequence_length,
                    &self.selected_samples,
                    &output_file,
                    &path,
                    beat_scale,
                ) {
                    println!("Error recording pattern: {:?}", e);
                }
            }
            Message::ToggleDrumSequence(on) => {
                self.playback_state.lock().unwrap().play_sequence_on = on;
                self.sequence_playing.store(on, Ordering::SeqCst);
                if on {
                    let stream_handle = Arc::clone(&self.stream_handle);
                    let playback_state = Arc::clone(&self.playback_state);
                    let beat_pattern_receiver = self.beat_pattern_receiver.clone();
                    let sequence_playing = Arc::clone(&self.sequence_playing);
                    let selected_samples = self.selected_samples.clone();
                    let path = self.root_sample_folder.clone() + "/";

                    let beat_scale = match self.sequence_scale {
                        SequenceScale::OneEighth => 2,
                        SequenceScale::OneSixteenth => 4,
                        SequenceScale::OneFourth => 1,
                    };

                    // Get the current beat pattern
                    let initial_beat_pattern =
                        self.sequence_state.lock().unwrap().beat_pattern.clone();

                    thread::spawn(move || {
                        play_pattern(
                            stream_handle,
                            playback_state,
                            beat_pattern_receiver,
                            sequence_playing,
                            selected_samples,
                            &path,
                            beat_scale,
                            initial_beat_pattern, // Pass the initial beat pattern
                        );
                    });
                } else {
                    // Ensure clean shutdown
                    self.sequence_playing.store(false, Ordering::SeqCst);
                    // Optionally, you can send a signal through beat_pattern_sender to wake up the receiver
                    // let _ = self.beat_pattern_sender.send(Vec::new());
                }
            }
            Message::UpdateSequenceLength(length) => {
                self.sequence_state.lock().unwrap().sequence_length = length * 2;
                for pattern in &mut self.sequence_state.lock().unwrap().beat_pattern {
                    pattern.resize((length * 2) as usize, false);
                }
            }
            Message::UpdateBeatPattern(file_index, beat_index, checked) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                if file_index < sequence_state.beat_pattern.len()
                    && beat_index < sequence_state.beat_pattern[file_index].len()
                {
                    sequence_state.beat_pattern[file_index][beat_index] = checked;
                    // Send updated beat pattern to playback thread
                    let _ = self
                        .beat_pattern_sender
                        .send(sequence_state.beat_pattern.clone());
                }
            }
            Message::UpdateBPM(bpm) => {
                self.playback_state.lock().unwrap().bpm = bpm;
            }
            Message::PlayAndAddSample(sample_name) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                let sequence_length = sequence_state.sequence_length as usize;

                if !self
                    .selected_samples
                    .values()
                    .any(|v| v.keys().next().unwrap() == &sample_name)
                {
                    let new_index = self.selected_samples.len();
                    let mut file_map = HashMap::new();
                    file_map.insert(sample_name.clone(), self.sample_folder.clone());

                    self.selected_samples.insert(new_index, file_map);

                    // Initialize the new beat pattern with the correct length
                    sequence_state
                        .beat_pattern
                        .push(vec![false; sequence_length]);
                }

                drop(sequence_state);
                let path = self.root_sample_folder.clone() + "/" + &self.sample_folder.to_string();
                play_audio(&self.stream_handle, sample_name.clone(), &path);
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let sequence_view = self.create_sequence_view();
        let sample_buttons = self.create_sample_buttons();
        let folder_pick_list: iced::widget::PickList<
            '_,
            SampleFolder,
            Vec<SampleFolder>,
            SampleFolder,
            Message,
            Theme,
            Renderer,
        > = PickList::new(
            self.sample_folders_options.clone(),
            Some(self.sample_folder.clone()),
            Message::ChangeSampleFolder,
        );

        let content = Column::new()
            .push(sequence_view)
            .push(Text::new("Sample Buttons").size(20))
            .push(folder_pick_list)
            .push(sample_buttons);

        scrollable(Container::new(content).width(Length::Fill).padding(20))
            .height(Length::Fill)
            .into()
    }
}
