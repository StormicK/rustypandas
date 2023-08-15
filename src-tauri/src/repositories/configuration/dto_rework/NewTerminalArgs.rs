use serde::{Serialize, Deserialize};

use super::{SchemePair, Color};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTerminalArgs {
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