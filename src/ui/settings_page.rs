use super::{MainUi, Message};
use iced::{
    widget::{checkbox, Column, Container, Text},
    Element, Length,
};

pub struct SettingsPage {
    pub is_dark_theme: bool,
}

impl SettingsPage {
    pub fn new(is_dark_theme: bool) -> Self {
        SettingsPage { is_dark_theme }
    }

    pub fn view(&self) -> Element<Message> {
        let theme_checkbox =
            checkbox("Dark Theme", self.is_dark_theme).on_toggle(Message::ToggleTheme);
        let content = Column::new()
            .spacing(20)
            .push(Text::new("Settings").size(30))
            .push(theme_checkbox);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
