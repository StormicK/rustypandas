use serde::{Serialize, Deserialize};

use super::ThemeColor::ThemeColor;
use super::ShowCloseButton::ShowCloseButton;

#[derive(Serialize, Deserialize, Debug)]
pub struct TabRowTheme {
    pub background: ThemeColor,
    pub unfocused_background: ThemeColor,
}