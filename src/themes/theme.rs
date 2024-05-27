use std::default;

use iced::{
    program::{Appearance, DefaultStyle},
    Color,
};

use super::colors;

#[derive(Default, Debug)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        match *self {
            Theme::Dark => Appearance {
                background_color: colors::DARK_GRAY,
                text_color: colors::OFF_WHITE,
            },
            Theme::Light => todo!(),
        }
    }
}
