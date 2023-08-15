use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScrollToMarkDirection {
    Previous,
    Next,
    First,
    Last,
}
