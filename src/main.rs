use iced::{Application, Settings};

use ui::midi_player::AudioPlayer;

mod scripts;
mod ui;

fn main() -> iced::Result {
    AudioPlayer::run(Settings::default())
}
