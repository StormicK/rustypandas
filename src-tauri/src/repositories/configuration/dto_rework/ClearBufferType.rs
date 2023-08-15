use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClearBufferType {
    All,
    Screen,
    Scrollback,
}
