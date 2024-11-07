use iced::{
    widget::{button, column, container, row, text, Space, Svg},
    Length,
};

use crate::{
    gui::Event,
    icons::Icon,
    themes::{
        button::ButtonType,
        container::ContainerType,
        rectangle::RectType,
        text::{TextType, INTER_FONT_BOLD},
    },
    type_defs::Element,
    widgets::rectangle::Rectangle,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Message {}

// TODO: While not avaliable for the master branch yet, iced_aw provides a slide widget that might be useful
pub struct ChangesPage;

impl ChangesPage {
    pub fn view<'a>() -> Element<'a, Message> {
        let change_test = Change {
            file_name: "Hello".into(),
            change_type: ChangeType::Both,
        };
        let sidebar = container(column![row![column![change_test.view(false)].padding(10)]])
            .width(300)
            .height(Length::Fill)
            .class(ContainerType::InContextSideBar);

        container(row![sidebar]).into()
    }

    pub fn update() -> Option<Event> {
        return None;
    }
}

enum ChangeType {
    Addition,
    Deletion,
    Both,
}
struct Change {
    // TODO: When implementation comes, look into how file name is received and possibly change the signature
    file_name: String,
    change_type: ChangeType,
}

const CHANGE_ICON_SIZE: u16 = 15;

impl Change {
    fn view<'a>(self, selected: bool) -> Element<'a, Message> {
        let icon_column = {
            let plus_icon = text("+")
                .size(20)
                .font(INTER_FONT_BOLD)
                .class(TextType::Addition);
            let minus_icon = text("-")
                .size(20)
                .font(INTER_FONT_BOLD)
                .class(TextType::Removal);

            match self.change_type {
                ChangeType::Addition => row![plus_icon],
                ChangeType::Deletion => row![minus_icon],
                ChangeType::Both => row![plus_icon, minus_icon].spacing(6),
            }
        };
        button(
            row![
                text(self.file_name).size(17).font(INTER_FONT_BOLD),
                Space::new(Length::Fill, Length::Shrink),
                icon_column
            ]
            .align_items(iced::Alignment::Center),
        )
        .padding([1, 5, 1, 5])
        .class(ButtonType::Change { selected: selected })
        .into()
    }
}
