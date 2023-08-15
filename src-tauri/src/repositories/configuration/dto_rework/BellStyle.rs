use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum VariantBEnum {
    Audible,
    Window,
    Taskbar,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VariantCEnum {
    Audible,
    Window,
    Taskbar,
    All,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BellStyle {
    VariantA(bool),
    VariantB(Vec::<VariantBEnum>),
    VariantC(Vec::<VariantCEnum>),
}