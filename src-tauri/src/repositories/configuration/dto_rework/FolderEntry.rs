use serde::{Serialize, Deserialize};

use super::NewTabMenuEntry::NewTabMenuEntryType;

//this one is a upcoming feature. so i wont bother :/
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderEntry {
    #[serde(rename = "type")]
    pub entry_type: NewTabMenuEntryType,
    pub name: String,
    pub icon: Icon,
    pub entries: Vec<NewTabMenuEntryType>,
    pub inline: FolderEntryInlining,
    pub allow_empty: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FolderEntryInlining {
    Never,
    Auto,
}
