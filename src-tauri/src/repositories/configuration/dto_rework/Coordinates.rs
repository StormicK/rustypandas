use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize, Serialize)]
struct Coordinates(String);