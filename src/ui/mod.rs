// mod.rs

pub mod arranger;
pub mod drum_machine_components;
pub mod drum_machine_page;
pub mod settings_components;
pub mod settings_page;
pub mod synth_page;
pub mod top_bar;

use std::sync::{Arc, Mutex};

use drum_machine_page::DrumMachine;
use iced::{
    widget::{Column, Text},
    Application, Command, Element, Theme,
};
use settings_page::SettingsPage;
use synth_page::SynthPage;

pub struct MainUi {
    current_page: Page,
    drum_machine: DrumMachine,
    settings_page: SettingsPage,
    pub sequence_state: Arc<Mutex<SequenceState>>,
    synth_page: SynthPage,
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
}

#[derive(Debug, Clone)]
pub enum Message {
    DrumMachineMessage(drum_machine_page::Message),
    ChangePage(Page),
    ToggleTheme(bool),
    SynthPageMessage(synth_page::Message),
    UpdateSequenceLength(u32),
}

impl Application for MainUi {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let sequence_state = Arc::new(Mutex::new(SequenceState {
            sequence_length: 16,
            beat_pattern: vec![vec![false; 16]; 12],
        }));

        let (drum_machine, drum_machine_command) = DrumMachine::new(sequence_state.clone());
        let synth_page = SynthPage::new(sequence_state.clone());

        (
            MainUi {
                current_page: Page::DrumMachine,
                drum_machine,
                settings_page: SettingsPage::new(true),
                synth_page,
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
            Message::SynthPageMessage(msg) => {
                self.synth_page.update(msg);
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
            Page::Synth => self.synth_page.view().map(Message::SynthPageMessage),
            Page::Settings => self.settings_page.view(),
        };

        Column::new().push(top_bar).push(content).into()
    }
}
