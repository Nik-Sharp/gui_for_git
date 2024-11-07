use std::any::Any;

use iced::{font, widget::{button, column, row, svg::Handle, text, Svg}, Font};

use crate::{icons::Icon, themes::{self, button::ButtonType}, type_defs::Element, views::{header::{self, Header}, pages::changes::{self, ChangesPage}}};

#[derive(Debug, Clone, PartialEq)]
pub enum PopUp {
    RepoSelect,
    BranchSelect,
    Profile,
    Settings
}
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Page {
    WelcomeScreen,
    #[default]
    Changes,
    Commits
}
#[derive(Debug, Clone)]
pub enum Message {
    Header(header::Message),
    ChangesPage(changes::Message)
}

pub enum Event {
    ChangeActivePage(Page),
    ChangeActivePopUp(Option<PopUp>)
}
#[derive(Default)]
pub struct Gui {
    page: Page,
    pop_up: Option<PopUp>
}

impl Gui {  
    pub fn view(&self) -> Element<'_, Message> {
        let header = Header::view(&self.page).map(Message::Header);

        let page = match self.page {
            Page::WelcomeScreen => todo!(),
            Page::Changes => ChangesPage::view().map(Message::ChangesPage),
            Page::Commits => todo!(),
        };

        column![
            header,
            page
        ].into()

    }

    pub fn update(&mut self, message: Message) {
        let event = match message {
            Message::Header(message) => Header::update(message),
            Message::ChangesPage(_) => todo!(),
        };

        if let Some(event) = event {
            match event {
                Event::ChangeActivePage(page) => self.page = page,
                Event::ChangeActivePopUp(pop_up) => self.pop_up = pop_up,
            };
        }
    }
}