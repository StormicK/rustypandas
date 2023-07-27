use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TerminalConfig {
    #[serde(rename = "$help")]
    pub help: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    pub actions: Vec<Action>,
    #[serde(rename = "copyFormatting")]
    pub copy_formatting: String,
    #[serde(rename = "copyOnSelect")]
    pub copy_on_select: bool,
    #[serde(rename = "defaultProfile")]
    pub default_profile: String,
    #[serde(rename = "newTabMenu")]
    pub new_tab_menu: Vec<NewTabMenu>,
    pub profiles: Profiles,
    pub schemes: Vec<Scheme>,
    pub themes: Vec<Theme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub command: Command,
    pub keys: String,
}

#[derive(Debug)]
pub enum Command {
    Copy(CopyCommand),
    Paste(String),
    Find(String),
    SplitPane(SplitPaneCommand),
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        // Perform custom deserialization logic for the `command` field
        if let Some(copy_command) = serde_json::from_value::<CopyCommand>(value.clone()).ok() {
            return Ok(Command::Copy(copy_command));
        }
        if let Some(paste) = serde_json::from_value::<String>(value.clone()).ok() {
            return Ok(Command::Paste(paste));
        }
        if let Some(find) = serde_json::from_value::<String>(value.clone()).ok() {
            return Ok(Command::Find(find));
        }
        if let Some(splitpane_command) = serde_json::from_value::<SplitPaneCommand>(value).ok() {
            return Ok(Command::SplitPane(splitpane_command));
        }

        Err(serde::de::Error::custom("Invalid command format"))
    }
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Command::Copy(copy_command) => copy_command.serialize(serializer),
            Command::Paste(paste) => paste.serialize(serializer),
            Command::Find(find) => find.serialize(serializer),
            Command::SplitPane(splitpane_command) => splitpane_command.serialize(serializer),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CopyCommand {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "singleLine")]
    pub single_line: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SplitPaneCommand {
    #[serde(rename = "action")]
    pub action: String,
    pub split: String,
    #[serde(rename = "splitMode")]
    pub split_mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTabMenu {
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profiles {
    pub defaults: DefaultProfile,
    pub list: Vec<Profile>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultProfile {
    // Add fields here if needed
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    #[serde(rename = "backgroundImage")]
    pub background_image: Option<String>,
    pub commandline: Option<String>,
    pub guid: String,
    pub hidden: bool,
    pub name: String,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scheme {
    pub background: String,
    pub black: String,
    pub blue: String,
    #[serde(rename = "brightBlack")]
    pub bright_black: String,
    #[serde(rename = "brightBlue")]
    pub bright_blue: String,
    #[serde(rename = "brightCyan")]
    pub bright_cyan: String,
    #[serde(rename = "brightGreen")]
    pub bright_green: String,
    #[serde(rename = "brightPurple")]
    pub bright_purple: String,
    #[serde(rename = "brightRed")]
    pub bright_red: String,
    #[serde(rename = "brightWhite")]
    pub bright_white: String,
    #[serde(rename = "brightYellow")]
    pub bright_yellow: String,
    #[serde(rename = "cursorColor")]
    pub cursor_color: String,
    pub cyan: String,
    pub foreground: String,
    pub green: String,
    pub name: String,
    pub purple: String,
    pub red: String,
    #[serde(rename = "selectionBackground")]
    pub selection_background: String,
    pub white: String,
    pub yellow: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub name: String,
    pub tab: Tab,
    pub window: Window,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tab {
    pub background: Option<String>,
    #[serde(rename = "showCloseButton")]
    pub show_close_button: String,
    #[serde(rename = "unfocusedBackground")]
    pub unfocused_background: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Window {
    #[serde(rename = "applicationTheme")]
    pub application_theme: String,
    #[serde(rename = "useMica")]
    pub use_mica: bool,
}
