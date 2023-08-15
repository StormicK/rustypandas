use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FindMatchDirection {
    Next,
    Prev,
}
