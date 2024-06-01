
use iced::
    widget::svg::{
        Catalog, 
        Status, 
        Style
    };
use crate::themes::theme::Theme;

use super::colors;


#[derive(Default)]
pub enum SvgType {
    #[default]
    Default
}
impl Catalog for Theme {
    type Class<'a> = SvgType;

    fn default<'a>() -> Self::Class<'a> {
        SvgType::default()
    }

    fn style(&self, class: &Self::Class<'_>, _status : Status) -> Style {

        match *self {
            Theme::Dark => match *class {
                SvgType::Default => Style {
                    color: Some(colors::OFF_WHITE)
                },
            },
            Theme::Light => todo!(),
        }

    }
}
