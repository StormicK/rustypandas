use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemePair {
    pub dark: String,
    pub light: String,
}