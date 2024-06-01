use iced::Color;

use crate::widgets::rectangle::{Catalog, Style};

use super::{colors, theme::Theme};

#[derive(Default)]
pub enum RectType {
    #[default]
    Default,
    Header,
}
impl Catalog for Theme {
    type Class<'a> = RectType;

    fn default<'a>() -> Self::Class<'a> {
        RectType::default()
    }
    fn style(&self, class: &Self::Class<'_>) -> Style {
        match *self {
            Theme::Dark => match *class {
                RectType::Default => Style {
                    color: Some(colors::OFF_WHITE),
                },
                RectType::Header => Style {
                    color: Some(colors::DARK_GRAY),
                },
            },
            Theme::Light => todo!(),
        }
    }
}
