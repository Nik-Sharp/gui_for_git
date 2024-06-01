mod themes;
mod gui;
mod type_defs;
mod views;
mod icons;
mod widgets;
use gui::Gui;
use iced::{Font, Settings};

static INTER_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/InterVariable.ttf").as_slice();
static INTER_FONT_BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf").as_slice();
fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        fonts: vec![INTER_FONT_BYTES.into(), INTER_FONT_BOLD_BYTES.into()],
        ..Settings::default()
    };
    iced::program("gui-for-git", Gui::update, Gui::view)
        .settings(settings)
        .run()
}
