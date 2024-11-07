use crate::themes::theme::Theme;

use iced::{
    font,
    widget::text::{Style, Catalog},
    Color, Font,
};

use super::colors;

pub const INTER_FONT: Font = Font::with_name("Inter");
//pub const INTER_FONT_BOLD: Font = Font::with_name("Inter Bold");
pub const INTER_FONT_BOLD: Font = Font {
    weight: font::Weight::Bold,
    ..Font::with_name("Inter")
};

#[derive(Default)]
pub enum TextType {
    #[default]
    Default,
    Addition,
    Removal
}
impl Catalog for Theme {
    type Class<'a> = TextType;

    fn default<'a>() -> Self::Class<'a> {
        TextType::default()
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        match *self {
            Theme::Dark => match *class {
                TextType::Default => Style {
                    color: Some(colors::OFF_WHITE),
                },
                TextType::Addition => Style {
                    color: Some(colors::DARK_GREEN),
                },
                TextType::Removal => Style {
                    color: Some(colors::RED),
                },
            },
            Theme::Light => todo!(),
        }
    }
}