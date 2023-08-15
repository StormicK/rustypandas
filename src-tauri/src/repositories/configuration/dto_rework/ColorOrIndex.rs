use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorOrIndex(String);