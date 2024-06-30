// mod.rs

pub mod arranger;
pub mod drum_machine_components;
pub mod drum_machine_page;
pub mod settings_components;
pub mod settings_page;
pub mod top_bar;

use drum_machine_page::DrumMachine;
use iced::{
    widget::{Column, Text},
    Application, Command, Element, Theme,
};
use settings_page::SettingsPage;

pub struct MainUi {
    current_page: Page,
    drum_machine: DrumMachine,
    settings_page: SettingsPage,
    pub is_dark_theme: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    DrumMachine,
    Arranger,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    DrumMachineMessage(drum_machine_page::Message),
    ChangePage(Page),
    ToggleTheme(bool),
}

impl Application for MainUi {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (drum_machine, drum_machine_command) = DrumMachine::new();
        (
            MainUi {
                current_page: Page::DrumMachine,
                drum_machine,
                settings_page: SettingsPage::new(true),
                is_dark_theme: true,
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
            Message::DrumMachineMessage(msg) => self
                .drum_machine
                .update(msg)
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
        }
    }

    fn view(&self) -> Element<Message> {
        let top_bar = self.create_top_bar();
        let content = match self.current_page {
            Page::DrumMachine => self.drum_machine.view().map(Message::DrumMachineMessage),
            Page::Arranger => Text::new("Arranger page (TODO)").into(),
            Page::Settings => self.settings_page.view(),
        };

        Column::new().push(top_bar).push(content).into()
    }
}
