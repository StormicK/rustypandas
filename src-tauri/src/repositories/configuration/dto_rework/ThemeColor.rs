use serde::{Serialize, Deserialize};

//basically all options are strings or null, but the json schema is more specific
#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeColor(Option::<String>);