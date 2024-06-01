use iced::{
    widget::{button, column, container, row, svg::Handle, Space, Svg, Text},
    Alignment, Length,
};

use crate::{
    gui::{Page, PopUp},
    icons::Icon,
    themes::{
        button::ButtonType, container::ContainerType, rectangle::RectType, svg::SvgType, text,
        theme::Theme,
    },
    type_defs::Element,
    widgets::rectangle::Rectangle,
};

const ICON_SIZE: u16 = 30;
#[derive(Debug, Clone)]
pub enum Message {
    PopUpOpen(PopUp),
    PullOrigin,
    PushOrigin,
    Commit,
    PageSwitch(Page),
}

pub struct Header;

// TODO: The left hand side begins to shrink as the windows gets smaller while there is still plenty of space on the right hand side
impl Header {
    pub fn view<'a>(current_page: &Page) -> Element<'a, Message> {
        let left = {
            let repository = button(
                row![
                    icon(Icon::Git, false, None),
                    Text::new("gui_for_git")
                        .font(text::INTER_FONT_BOLD)
                        .size(20),
                    icon(Icon::DownArrow, false, None),
                ]
                .align_items(Alignment::Center)
                .spacing(10),
            )
            .on_press(Message::PopUpOpen(PopUp::RepoSelect));

            let branch = button(
                row![
                    icon(Icon::Branch, false, None),
                    Text::new("master").font(text::INTER_FONT_BOLD).size(20),
                    icon(Icon::DownArrow, false, None),
                ]
                .align_items(Alignment::Center)
                .spacing(10),
            )
            .on_press(Message::PopUpOpen(PopUp::BranchSelect));
            row![
                repository,
                separator(),
                branch,
                Space::new(Length::Fill, Length::Shrink),
            ]
            .spacing(14)
        };

        let center = {
            let changes = icon(
                Icon::Changes,
                *current_page == Page::Changes,
                Some(Message::PageSwitch(Page::Changes)),
            );

            let commits = icon(
                Icon::Clock,
                *current_page == Page::Commits,
                Some(Message::PageSwitch(Page::Commits).into()),
            );

            // TODO: Set up changing when origin is being pulled, and hide it if there's no origin
            let pull_origin = icon(Icon::PullOrigin, false, Some(Message::PullOrigin));
            
            // TODO: same as before
            let push_origin = icon(Icon::PushOrigin, false, Some(Message::PushOrigin));

            row![
                changes,
                separator(),
                commits,
                separator(),
                pull_origin,
                separator(),
                push_origin,
            ].align_items(Alignment::Center).spacing(16)
        };

        let right = {
            let settings = icon(Icon::Settings, false, Some(Message::PopUpOpen(PopUp::Settings)));
            let profile = icon(Icon::Profile, false, Some(Message::PopUpOpen(PopUp::Profile)));

            row![
                Space::new(Length::Fill, Length::Shrink),
                profile,
                separator(),
                settings,
            ].align_items(Alignment::Center).spacing(14)
        };

        container(row![left, center, right].spacing(10))
            .width(Length::Fill)
            .height(60)
            .padding(10)
            .class(ContainerType::Header)
            .into()
    }
}

fn separator<'a>() -> Element<'a, Message> {
    Rectangle::new(3, Length::Fill, 0.0).class(RectType::Header).into()
}
fn icon<'a>(icon: Icon, selected: bool, message: Option<Message>) -> Element<'a, Message> {
    let mut icon_column = column![Svg::from(icon).height(ICON_SIZE).width(ICON_SIZE)].spacing(6);

    // Adds the little bar under the icon to indicate it's selected
    if selected {
        icon_column = icon_column.push(Rectangle::new(ICON_SIZE, 2, 25.0));
    }

    //icon_column.into()
    button(icon_column).padding(0).on_press_maybe(message).into()
}
