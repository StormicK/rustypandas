use serde::{Serialize, Deserialize};

use super::TabTheme::TabTheme;
use super::TabRowTheme::TabRowTheme;
use super::WindowTheme::WindowTheme;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub tab: TabTheme,
    pub tab_row: TabRowTheme,
    pub window: WindowTheme,
}