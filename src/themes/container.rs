use iced::widget::container::{Catalog, Style};

use crate::themes::theme::Theme;

use super::colors;

#[derive(Default)]
pub enum ContainerType {
    #[default]
    NoBackground,
    Header,
    // TODO: Does this name suck, for the sidebar inside the change / history screen
    InContextSideBar
}

impl Catalog for Theme {
    type Class<'a> = ContainerType;

    fn default<'a>() -> Self::Class<'a> {
        ContainerType::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        match *self {
            Theme::Dark => match *class {
                ContainerType::NoBackground => Style::with_background(Style::default(), colors::TRANSPARENT),
                ContainerType::Header => Style::with_background(Style::default(), colors::DARK_BLACK),
                ContainerType::InContextSideBar => Style::with_background(Style::default(), colors::BLUE_GRAY),
            },
            Theme::Light => todo!(),
        }
    }
}