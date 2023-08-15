// use serde::{ Serialize, Deserialize };

// #[derive(Debug, Deserialize, Serialize)]
// pub struct TerminalConfig {
//     #[serde(rename = "$help")]
//     pub help: String,
//     #[serde(rename = "$schema")]
//     pub schema: String,
//     pub actions: Vec<Action>,
//     #[serde(rename = "copyFormatting")]
//     pub copy_formatting: String,
//     #[serde(rename = "copyOnSelect")]
//     pub copy_on_select: bool,
//     #[serde(rename = "defaultProfile")]
//     pub default_profile: String,//required
//     #[serde(rename = "newTabMenu")]
//     pub new_tab_menu: Vec<NewTabMenu>,
//     pub profiles: Profiles,//required
//     pub schemes: Vec<Scheme>,//required
//     pub theme: Option<String>,
//     pub themes: Vec<Theme>,
// }