use iced::widget::{svg::Handle, Svg};

use crate::themes::theme::Theme;

static GIT_LOGO : &'static [u8] = include_bytes!("../assets/icons/git-logo.svg");
static BRANCH_LOGO : &'static [u8] = include_bytes!("../assets/icons/branch.svg");
static DOWNWARD : &'static [u8] = include_bytes!("../assets/icons/caret-down-outline.svg");

static CHANGES_ICON : &'static [u8] = include_bytes!("../assets/icons/changes.svg");
static CLOCK_ICON : &'static [u8] = include_bytes!("../assets/icons/clock.svg");
static PULL_ORIGIN_ICON : &'static [u8] = include_bytes!("../assets/icons/pull-origin.svg");
static COMMIT_ICON : &'static [u8] = include_bytes!("../assets/icons/commit.svg");
static PUSH_ORIGIN_ICON : &'static [u8] = include_bytes!("../assets/icons/upload.svg");

static PROFILE_ICON : &'static [u8] = include_bytes!("../assets/icons/profile.svg");
static SETTINGS_ICON : &'static [u8] = include_bytes!("../assets/icons/settings.svg");

static MINUS_ICON : &'static [u8] = include_bytes!("../assets/icons/minus.svg");
static PLUS_ICON : &'static [u8] = include_bytes!("../assets/icons/plus.svg");

pub enum Icon {
    Git,
    Branch,
    DownArrow,

    Changes,
    Clock,
    PullOrigin,
    PushOrigin,
    Commit,

    Profile,
    Settings,

    Minus,
    Plus,
}

impl<'a> From<Icon> for Svg<'a, Theme> {
    fn from(icon: Icon) -> Svg<'a, Theme> {
        let icon_bytes = match icon {
            Icon::Git => GIT_LOGO,
            Icon::Branch => BRANCH_LOGO,
            Icon::DownArrow => DOWNWARD,
            Icon::Changes => CHANGES_ICON,
            Icon::Clock => CLOCK_ICON,
            Icon::PullOrigin => PULL_ORIGIN_ICON,
            Icon::PushOrigin => PUSH_ORIGIN_ICON,
            Icon::Commit => COMMIT_ICON,
            Icon::Profile => PROFILE_ICON,
            Icon::Settings => SETTINGS_ICON,
            Icon::Minus => MINUS_ICON,
            Icon::Plus => PLUS_ICON,
        };

        Svg::new(Handle::from_memory(icon_bytes)).into()

    }
}