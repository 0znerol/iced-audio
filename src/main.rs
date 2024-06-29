use iced::{Application, Settings};

use ui::main_ui::AudioPlayer;

mod scripts;
mod ui;

fn main() -> iced::Result {
    AudioPlayer::run(Settings::default())
}
