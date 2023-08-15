use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AnchorKey {
    Ctrl,
    Alt,
    Shift,
}