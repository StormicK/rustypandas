use serde::{Serialize, Deserialize};

use super::ThemeColor::ThemeColor;
use super::ShowCloseButton::ShowCloseButton;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TabTheme {
    pub background: ThemeColor,
    pub unfocused_background: ThemeColor,
    pub show_close_button: ShowCloseButton,
}