use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub enum BellSound {
    VariantA(Option::<String>),
    VariantB(Vec::<String>),
}