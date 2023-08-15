use serde::{Serialize, Deserialize};

use super::FocusDirection;
use super::ShortcutActionName::ShortcutActionName;
use super::CopyFormat::CopyFormat;
use super::Color::Color;

pub struct Action(ShortcutActionName);

pub struct AdjustFontSizeAction {
    pub action: Action,
    pub delta: i32
}

pub struct CopyAction {
    pub action: Action,
    pub single_line: bool,
    pub dismiss_selection: bool,
    pub copy_formatting: Option::<CopyFormat>
}

//new tab action
pub struct NewTabAction {
    pub action: Action,
    pub command_line: String,
    pub tab_title: String,
    pub starting_directory: String,
    pub profile: String,
    pub index: u32,
    pub tab_color: Color::Color,
    pub suppress_application_title: bool,
    pub color_scheme: String,
    pub elevate: bool,
}

pub struct SwitchToTabAction {
    pub action: Action,
    pub index: u32,
}

pub struct MovePaneAction {
    pub action: Action,
    pub index: u32,
}

use super::FocusDirection::FocusDirection;
pub struct MoveFocusAction {
    pub action: Action,
    pub direction: FocusDirection
}

//following... just wanna get it to work now lol