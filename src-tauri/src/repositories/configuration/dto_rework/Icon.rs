use serde::{ Serialize, Deserialize };

//can also be an emoji according to ms 😎
#[derive(Debug, Deserialize, Serialize)]
pub struct Icon(Option::<String>);