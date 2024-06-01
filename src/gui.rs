use std::any::Any;

use iced::{font, widget::{button, column, row, svg::Handle, text, Svg}, Font};

use crate::{icons::Icon, themes::{self, button::ButtonType}, type_defs::Element, views::header::{self, Header}};

#[derive(Debug, Clone, PartialEq)]
pub enum PopUp {
    RepoSelect,
    BranchSelect,
    Profile,
    Settings
}
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Page {
    #[default]
    WelcomeScreen,
    Changes,
    Commits
}
#[derive(Debug, Clone)]
pub enum Message {
    Header(header::Message)
}

#[derive(Default)]
pub struct Gui {
    page: Page
}

impl Gui {  
    pub fn view(&self) -> Element<'_, Message> {
        let header = Header::view(&self.page).map(Message::Header);

        column![
            header,
        ].into()

    }

    pub fn update(&mut self, message: Message) {

    }
}