use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchToAdjacentTabArgs(Option::<SwitchType>);

#[derive(Debug, Serialize, Deserialize)]
pub enum SwitchType {
    Mru,
    InOrder,
    Disabled,
}