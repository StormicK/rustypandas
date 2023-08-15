use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Previous,
    NextInOrder,
    PreviousInOrder,
    First,
    Parent,
    Child,
}