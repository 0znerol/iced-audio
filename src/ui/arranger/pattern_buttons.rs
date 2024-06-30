// use iced::{
//     widget::{checkbox, Button, Column, Row, Text},
//     Length,
// };

// use crate::ui::drum_machine_page::{AudioPlayer, Message};

// impl AudioPlayer {
//     pub fn create_pattern_buttons(&self) -> Column<Message> {
//         self.audio_files
//             .chunks(4)
//             .fold(Column::new().spacing(10), |column, chunk| {
//                 let row = chunk.iter().fold(Row::new().spacing(10), |row, file_name| {
//                     row.push(
//                         Button::new(Text::new(file_name))
//                             .on_press(Message::PlayAndAddSample(file_name.clone()))
//                             .padding(5)
//                             .width(Length::FillPortion(1))
//                             .height(Length::Fixed(80.0)),
//                     )
//                 });
//                 column.push(row)
//             })
//     }
// }
