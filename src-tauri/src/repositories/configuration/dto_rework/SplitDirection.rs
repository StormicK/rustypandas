use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SplitDirection {
    Auto,
    Up,
    Right,
    Down,
    Left,
}
