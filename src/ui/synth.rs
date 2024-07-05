use iced::{
    widget::{Button, Checkbox, Column, PickList, Row, Text},
    Command, Element, Length, Renderer, Theme,
};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::{drum_machine::SequenceScale, drum_machine_components::sequence_view, SequenceState};

pub struct Synth {
    sequence_state: Arc<Mutex<SequenceState>>,
    notes: Vec<String>,
    pub is_playing: Arc<Mutex<bool>>,
    play_sender: mpsc::Sender<bool>,
    pub sequence_scale_options: Vec<SequenceScale>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleNote(usize, usize, bool),
    PlaySequence,
    StopSequence,
    PlaybackFinished,
    ChangeSequenceScale(SequenceScale),
    ChangeFrequency(f32),
}

impl Synth {
    pub fn new(sequence_state: Arc<Mutex<SequenceState>>) -> Self {
        let notes: Vec<_> = vec![
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let (play_sender, play_receiver) = mpsc::channel();

        let is_playing = Arc::new(Mutex::new(false));
        let is_playing_clone = is_playing.clone();
        let sequence_state_clone = sequence_state.clone();
        let sequence_scale_options = vec![
            SequenceScale::OneFourth,
            SequenceScale::OneEighth,
            SequenceScale::OneSixteenth,
        ];

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

        Synth {
            sequence_state,
            notes,
            is_playing,
            play_sender,
            sequence_scale_options,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangeFrequency(frequency) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                sequence_state.frequency = frequency;
                return Command::none();
            }
            Message::ChangeSequenceScale(new_sequence_size) => {
                self.sequence_state.lock().unwrap().synth_scale = new_sequence_size;
                return Command::none();
            }
            Message::ToggleNote(note_index, beat_index, checked) => {
                let mut sequence_state = self.sequence_state.lock().unwrap();
                sequence_state.note_pattern[note_index][beat_index] = checked;
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

    pub fn view(&self) -> Element<Message> {
        let sequence_state = self.sequence_state.lock().unwrap();
        let note_pattern = &sequence_state.note_pattern;
        let sequence_length_pick_list: iced::widget::PickList<
            '_,
            SequenceScale,
            Vec<SequenceScale>,
            SequenceScale,
            Message,
            Theme,
            Renderer,
        > = PickList::new(
            self.sequence_scale_options.clone(),
            Some(sequence_state.synth_scale.clone()),
            Message::ChangeSequenceScale,
        );
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

        let frequency_slider: iced::widget::Slider<'_, f32, Message, Theme> =
            iced::widget::Slider::new(
                100.0..=1000.0,
                sequence_state.frequency,
                Message::ChangeFrequency,
            );

        Column::new()
            .push(Row::new().push(sequence_length_pick_list).push(play_button))
            .push(sequence_view)
            .push(
                Row::new()
                    .push(Text::new("frequency: "))
                    .push(frequency_slider)
                    .width(Length::Fixed(500.0))
                    .push(Text::new(format!("{:.2}", sequence_state.frequency)))
                    .spacing(20),
            )
            .into()
    }
}
