use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MoveTabDirection {
    Forward,
    Backward,
}
