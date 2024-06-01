use crate::themes::theme::Theme;
use iced::Renderer;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme, Renderer>;