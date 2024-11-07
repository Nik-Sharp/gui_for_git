use crate::themes::theme::Theme;
use super::colors;

use iced::{widget::button::{Catalog, Status, Style}, Border};

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Empty,
    Change { selected: bool }
}

impl Catalog for Theme {
    type Class<'a> = ButtonType;

    fn default<'a>() -> Self::Class<'a> {
        ButtonType::default()
    }

    // TODO: Status is ignored at the moment, but we could possibly add some change if the button is hovered
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        match *self {
            Theme::Dark => match *class {
                ButtonType::Empty => Style::with_background(Style::default(), colors::TRANSPARENT),
                ButtonType::Change {selected}=> if status == Status::Hovered || selected {
                    Style { background: Some(colors::GRAY.into()), border: Border::rounded(5), ..Style::default() }
                }
                else {
                    Style { background: Some(colors::DARK_GRAY.into()), border: Border::rounded(5), ..Style::default() }
                }
            },
            Theme::Light => todo!(),
        }
    }
}