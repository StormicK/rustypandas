use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemePair {
    pub light: String,
    pub dark: String
}