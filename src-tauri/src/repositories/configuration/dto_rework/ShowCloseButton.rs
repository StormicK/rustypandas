use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ShowCloseButton {
    Always,
    Hover,
    Never,
    ActiveOnly,
}
