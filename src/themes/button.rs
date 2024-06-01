use crate::themes::theme::Theme;
use super::colors;

use iced::widget::button::{Catalog, Status, Style};

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Empty,
    Gray
    
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
                ButtonType::Gray => Style::with_background(Style::default(), colors::GRAY),
            },
            Theme::Light => todo!(),
        }
    }
}