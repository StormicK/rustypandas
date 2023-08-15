use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResizeDirection {
    Left,
    Right,
    Up,
    Down,
}