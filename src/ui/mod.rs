// mod.rs

pub mod arranger;
pub mod drum_machine;
pub mod drum_machine_components;
pub mod settings_components;
pub mod settings_page;
pub mod synth;
pub mod top_bar;

use std::sync::{Arc, Mutex};

use drum_machine::{DrumMachine, SequenceScale};
use iced::{
    command,
    widget::{Column, Text},
    Application, Command, Element, Theme,
};
use settings_page::SettingsPage;
use synth::Synth;

pub struct MainUi {
    current_page: Page,
    drum_machine: DrumMachine,
    settings_page: SettingsPage,
    pub sequence_state: Arc<Mutex<SequenceState>>,
    synth: Synth,
    pub is_dark_theme: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    DrumMachine,
    Synth,
    Arranger,
    Settings,
}

pub struct SequenceState {
    pub sequence_length: u32,
    pub beat_pattern: Vec<Vec<bool>>,
    pub note_pattern: Vec<Vec<bool>>,
    pub bpm: u32,
    pub drum_scale: SequenceScale,
    pub synth_scale: SequenceScale,
    pub drum_sequence_on: bool,
    pub synth_sequence_on: bool,
    pub octave: u32,
    pub frequency: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    DrumMachineMessage(drum_machine::Message),
    ChangePage(Page),
    ToggleTheme(bool),
    SynthMessage(synth::Message),
    UpdateSequenceLength(u32),
    UpdateBpm(u32),
    StartBothSequences(bool),
}

impl Application for MainUi {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let sequence_state = Arc::new(Mutex::new(SequenceState {
            sequence_length: 16,
            beat_pattern: vec![vec![false; 16]; 0],
            note_pattern: vec![vec![false; 32]; 12],
            bpm: 120,
            drum_scale: SequenceScale::OneEighth,
            synth_scale: SequenceScale::OneFourth,
            drum_sequence_on: false,
            synth_sequence_on: false,
            octave: 0,
            frequency: 440.0,
        }));

        let (drum_machine, drum_machine_command) = DrumMachine::new(sequence_state.clone());
        let synth = Synth::new(sequence_state.clone());

        (
            MainUi {
                current_page: Page::DrumMachine,
                drum_machine,
                settings_page: SettingsPage::new(true),
                synth,
                is_dark_theme: true,
                sequence_state,
            },
            drum_machine_command.map(Message::DrumMachineMessage),
        )
    }

    fn theme(&self) -> Theme {
        if self.is_dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn title(&self) -> String {
        String::from("DAW")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartBothSequences(play_sequence) => {
                if play_sequence {
                    let _ = self
                        .drum_machine
                        .update(drum_machine::Message::PlaySequence)
                        .map(Message::DrumMachineMessage);
                    let _ = self.synth.update(synth::Message::PlaySequence);
                    //migth cause deadlock
                    self.sequence_state.lock().unwrap().drum_sequence_on = true;
                    self.sequence_state.lock().unwrap().synth_sequence_on = true;
                } else {
                    let _ = self
                        .drum_machine
                        .update(drum_machine::Message::StopSequence)
                        .map(Message::DrumMachineMessage);
                    let _ = self.synth.update(synth::Message::StopSequence);
                    //migth cause deadlock
                    self.sequence_state.lock().unwrap().drum_sequence_on = false;
                    self.sequence_state.lock().unwrap().synth_sequence_on = false;
                }
                Command::none()
            }
            Message::UpdateBpm(bpm) => {
                self.sequence_state.lock().unwrap().bpm = bpm;
                Command::none()
            }
            Message::SynthMessage(msg) => {
                self.synth.update(msg);
                Command::none()
            }
            Message::DrumMachineMessage(msg) => self
                .drum_machine
                .update(msg) // Pass current_page here
                .map(Message::DrumMachineMessage),
            Message::ChangePage(page) => {
                self.current_page = page;
                Command::none()
            }
            Message::ToggleTheme(mut theme_bool) => {
                theme_bool = !self.is_dark_theme;
                self.settings_page.is_dark_theme = theme_bool;
                self.is_dark_theme = theme_bool;
                Command::none()
            }
            Message::UpdateSequenceLength(length) => {
                self.sequence_state.lock().unwrap().sequence_length = length * 2;
                for pattern in &mut self.sequence_state.lock().unwrap().beat_pattern {
                    pattern.resize((length * 2) as usize, false);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let top_bar = self.create_top_bar();
        let content = match self.current_page {
            Page::DrumMachine => self.drum_machine.view().map(Message::DrumMachineMessage),
            Page::Arranger => Text::new("Arranger page (TODO)").into(),
            Page::Synth => self.synth.view().map(Message::SynthMessage),
            Page::Settings => self.settings_page.view(),
        };

        Column::new().push(top_bar).push(content).into()
    }
}
