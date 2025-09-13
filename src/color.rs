use csscolorparser::Color;

#[derive(Debug, thiserror::Error)]
#[error("Invalid color string: {0}")]

pub struct ParseColorError(String);

pub fn parse(s: &str) -> Result<Color, ParseColorError> {
    let c = csscolorparser::parse(s);
    c.map_err(|_| ParseColorError(s.to_string()))
}

pub enum ColorFormat{
    Hex,
    CssRgb,
}

pub fn to_format(color: &Color, format: ColorFormat) -> String {
    use ColorFormat::*;
    match format {
        Hex => color.to_css_hex(),
        CssRgb => color.to_css_rgb(),
    }
} 
