use serde::{ Serialize, Deserialize };

use super::{SchemePair, Color};

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorScheme {
    VariantA(SchemePair::SchemePair),
    VariantB(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CursorShape {
    Bar,
    DoubleUnderscore,
    EmptyBox,
    FilledBox,
    Underscore,
    Vintage
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackgroundImageEnum {
    VariantA(Option::<String>),
    DesktopWallpaper    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackgroundImageStretchmodeEnum {
    Fill,
    None,
    Uniform,
    UniformToFill    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackgroundImageAlignmentEnum {
    Bottom,
    BottomLeft,
    BottomRight,
    Center,
    Left,
    Right,
    Top,
    TopLeft,
    TopRight,    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IntenseTextStyleEnum {
    None,
    Bold,
    Bright,
    All,    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AdjustIndistinguishableColorsEnum {
    Never,
    Indexed,
    Always,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppearanceConfig {
    color_scheme: ColorScheme,
    foreground: Color::Color,
    background: Color::Color,
    selection_background: Option::<Color::Color>,
    cursor_color: Option::<Color::Color>,
    cursor_shape: CursorShape,
    cursor_height: Option::<i32>,
    background_image: BackgroundImageEnum,
    background_image_opacity: Option::<f64>,
    background_image_stretchmode: BackgroundImageStretchmodeEnum,
    background_image_alignment: BackgroundImageAlignmentEnum,
    intense_text_style: IntenseTextStyleEnum,
    adjust_indistinguishable_colors: AdjustIndistinguishableColorsEnum,
    #[serde(rename(serialize = "experimental.retroTerminalEffect", deserialize = "experimental.retroTerminalEffect"))]
    retro_terminal_effect: bool,
    #[serde(rename(serialize = "experimental.pixelShaderPath", deserialize = "experimental.pixelShaderPath"))]
    pixel_shader_path: String
}