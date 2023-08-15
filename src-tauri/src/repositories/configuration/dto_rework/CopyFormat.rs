use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CopyFormat {
    Boolean(bool),
    Array(Vec<CopyFormatItem>),
    String(CopyFormatString),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CopyFormatItem {
    Html,
    Rtf,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CopyFormatString {
    Html,
    Rtf,
    All,
    None,
}