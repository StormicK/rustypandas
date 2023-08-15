use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NewTabMenuEntryType {
    Source,
    Profile,
    Folder,
    Separator,
    RemainingProfiles,
    MatchProfiles,
}
