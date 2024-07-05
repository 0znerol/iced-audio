use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex, RwLock,
    },
    thread,
    time::Duration,
};

use iced::{
    widget::{checkbox, scrollable, slider, Button, Column, Container, PickList, Row, Text},
    Command, Element, Length, Renderer, Theme,
};
use rodio::{OutputStream, OutputStreamHandle};

use crate::scripts::record_pattern::record_pattern;

use super::{MainUi, Page, SequenceState};

pub struct DrumMachine {
    output_stream: OutputStream,
    stream_handle: Arc<rodio::OutputStreamHandle>,
    pub audio_files: Vec<String>,
    play_sender: mpsc::Sender<bool>,
    pub is_playing: Arc<Mutex<bool>>,
    pub selected_samples: Arc<RwLock<BTreeMap<usize, HashMap<String, SampleFolder>>>>,
    pub sequence_scale_options: Vec<SequenceScale>,
    pub sequence_scale: SequenceScale,
    pub playback_state: Arc<Mutex<PlaybackState>>,
    pub beat_pattern_sender: crossbeam_channel::Sender<Vec<Vec<bool>>>,
    pub beat_pattern_receiver: crossbeam_channel::Receiver<Vec<Vec<bool>>>,
    pub root_sample_folder: String,
    pub sample_folders_options: Vec<SampleFolder>,
    pub sample_folder: SampleFolder,
    pub add_sample_on_play: bool,
    pub sequence_state: Arc<Mutex<SequenceState>>,
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

pub struct PlaybackState {
    pub play_sequence_on: bool,
    // pub bpm: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    // ToggleDrumSequence(bool),
    UpdateBeatPattern(usize, usize, bool),
    // UpdateBPM(u32),
    PlayAndAddSample(String),
    RecordPattern,
    ChangeSequenceScale(SequenceScale),
    RemoveSample(usize),
    ChangeSampleFolder(SampleFolder),
    ToggleAddSampleOnPlay(bool),
    PlaySequence,
    StopSequence,
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
    pub fn new(sequence_state: Arc<Mutex<SequenceState>>) -> (Self, Command<Message>) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let audio_files = Self::get_audio_files("drumKits/909");
        let (play_sender, play_receiver) = mpsc::channel();
        let is_playing = Arc::new(Mutex::new(false));
        let is_playing_clone = is_playing.clone();
        let selected_samples = Arc::new(RwLock::new(BTreeMap::new()));
        let selected_samples_clone = selected_samples.clone();
        let sequence_state_clone = sequence_state.clone();
        let sequence_scale_options = vec![
            SequenceScale::OneFourth,
            SequenceScale::OneEighth,
            SequenceScale::OneSixteenth,
        ];
        let sequence_scale = sequence_state.lock().unwrap().drum_scale; //might cause deadlock

        let (beat_pattern_sender, beat_pattern_receiver) = crossbeam_channel::unbounded();

        let playback_state = Arc::new(Mutex::new(PlaybackState {
            play_sequence_on: false,
        }));
        let root_sample_folder = "drumKits".to_string();
        let root_sample_folder_clone = root_sample_folder.clone();
        let sample_folders_options = vec![SampleFolder::NineONine, SampleFolder::EightOEight];
        let sample_folder = SampleFolder::NineONine;
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
                            Self::play_pattern(
                                sequence_state_clone.clone(),
                                is_playing_clone.clone(),
                                stream_handle,
                                selected_samples_clone.clone(),
                                &root_sample_folder_clone,
                            );
                        }
                    } else {
                        *is_playing_clone.lock().unwrap() = false;
                        stream_option = None;
                    }
                }
            }
        });
        (
            DrumMachine {
                output_stream: stream,
                stream_handle: Arc::new(stream_handle),
                audio_files,
                playback_state,
                beat_pattern_sender,
                beat_pattern_receiver,
                play_sender,
                is_playing,
                selected_samples,
                sequence_scale_options,
                sequence_scale,
                root_sample_folder,
                sample_folders_options,
                sample_folder,
                add_sample_on_play: false,
                sequence_state,
            },
            Command::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlaySequence => {
                let mut is_playing = self.is_playing.lock().unwrap();
                if !*is_playing {
                    *is_playing = true;
                    self.play_sender.send(true).unwrap();
                }
                return Command::none();
            }
            Message::StopSequence => {
                let mut is_playing = self.is_playing.lock().unwrap();
                *is_playing = false;
                self.play_sender.send(false).unwrap();
                return Command::none();
            }
            Message::ToggleAddSampleOnPlay(checked) => {
                self.add_sample_on_play = checked;
            }
            Message::ChangeSampleFolder(folder) => {
                self.sample_folder = folder.clone();
                self.audio_files =
                    Self::get_audio_files(&format!("{}/{}", self.root_sample_folder, folder));
            }
            Message::RemoveSample(index) => {
                let mut selected_samples = self.selected_samples.write().unwrap();
                if selected_samples.contains_key(&index) {
                    selected_samples.remove(&index);

                    // Reindex the remaining samples
                    let new_samples: BTreeMap<usize, HashMap<String, SampleFolder>> =
                        selected_samples.values().cloned().enumerate().collect();
                    *selected_samples = new_samples;
                }

                let mut sequence_state = self.sequence_state.lock().unwrap();
                if index < sequence_state.beat_pattern.len() {
                    sequence_state.beat_pattern.remove(index);
                }
            }
            Message::ChangeSequenceScale(new_sequence_size) => {
                self.sequence_state.lock().unwrap().drum_scale = new_sequence_size;
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
                    self.sequence_state.lock().unwrap().bpm, //might cause freeze when recording
                    sequence_state.sequence_length,
                    &self.selected_samples,
                    &output_file,
                    &path,
                    beat_scale,
                ) {
                    println!("Error recording pattern: {:?}", e);
                }
            }
            Message::UpdateBeatPattern(file_index, beat_index, checked) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                if file_index < sequence_state.beat_pattern.len()
                    && beat_index < sequence_state.beat_pattern[file_index].len()
                {
                    sequence_state.beat_pattern[file_index][beat_index] = checked;
                    let _ = self
                        .beat_pattern_sender
                        .send(sequence_state.beat_pattern.clone());
                }
            }
            Message::PlayAndAddSample(sample_name) => {
                if self.add_sample_on_play {
                    let mut sequence_state = self.sequence_state.lock().unwrap();
                    let sequence_length = sequence_state.sequence_length as usize;

                    let mut selected_samples = self.selected_samples.write().unwrap();
                    if !selected_samples
                        .values()
                        .any(|v| v.keys().next().unwrap() == &sample_name)
                    {
                        let new_index = selected_samples.len();
                        let mut file_map = HashMap::new();
                        file_map.insert(sample_name.clone(), self.sample_folder.clone());

                        selected_samples.insert(new_index, file_map);

                        sequence_state
                            .beat_pattern
                            .push(vec![false; sequence_length]);
                    }
                    drop(selected_samples);
                    drop(sequence_state);
                }

                // Play the sample (unchanged)
                let note_duration = Duration::from_millis((60_000 / 120) as u64);
                let path = self.root_sample_folder.clone() + "/" + &self.sample_folder.to_string();
                let stream_handle = Arc::new(self.stream_handle.clone());
                thread::spawn(move || {
                    Self::play_audio(&stream_handle, note_duration, sample_name.clone(), &path);
                });
                // play_audio(
                //     &self.stream_handle,
                //     note_duration,
                //     sample_name.clone(),
                //     &path,
                // );
            }
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let sequence_view = self.create_sequence_view();
        let sample_buttons = self.create_sample_buttons();

        let content = Column::new().push(sequence_view).push(sample_buttons);

        scrollable(Container::new(content).width(Length::Fill).padding(20))
            .height(Length::Fill)
            .into()
    }
}
