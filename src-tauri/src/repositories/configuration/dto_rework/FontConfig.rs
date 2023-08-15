use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum WeightVariantAEnum {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack
}

impl WeightVariantAEnum {
    pub const EXTRA_LIGHT: &'static str = "extra-light";
    pub const SEMI_BOLD: &'static str = "semi-bold";
    pub const EXTRA_BOLD: &'static str = "extra-bold";
    pub const EXTRA_BLACK: &'static str = "extra-black";
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WeightEnum {
    VariantA(WeightVariantAEnum),
    VariantB(u32),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FontConfig {
    face: String,
    size: u32,
    weight: WeightEnum
    //skipping features and axes. lets find out what will happen
    
    //TODO: these wont be skipped: cellwidht cellheight #/$defs/CSSLengthPercentage

}