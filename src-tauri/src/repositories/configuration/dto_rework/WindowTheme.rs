use serde::{Serialize, Deserialize};

use super::ThemeColor::ThemeColor;
use super::ShowCloseButton::ShowCloseButton;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationThemeEnum {
    Light,
    Dark,
    System
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowTheme {
    pub application_theme: ApplicationThemeEnum,
    pub use_mica: bool,
    #[serde(rename(serialize = "experimental.rainbowFrame", deserialize = "experimental.rainbowFrame"))]
    pub rainbow_frame: bool,  
    pub frame: ThemeColor,
    pub unfocused_frame: ThemeColor,
}