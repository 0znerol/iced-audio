use iced::{Application, Settings};
use ui::MainUi;

mod scripts;
mod ui;

fn main() -> iced::Result {
    MainUi::run(Settings::default())
}
