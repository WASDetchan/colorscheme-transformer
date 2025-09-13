use csscolorparser::Color;
use std::{collections::HashMap, error::Error};

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

#[derive(Debug)]
pub struct Colorset {
    pub colors: HashMap<String, Color>,
}

impl Colorset {
    pub fn from_yaml_str(colors_yaml: &str) -> Result<Self, Box<dyn Error>> {
        let colors_strings: HashMap<String, String> = serde_yml::from_str(colors_yaml)
            .map_err(|e| format!("Invalid colorset file: {}", e))?;
        let scheme = Self {
            colors: colors_strings
                .into_iter()
                .map(|(name, color_string)| {
                    Ok::<(std::string::String, Color), Box<dyn Error>>((
                        name,
                       
                        parse(color_string.as_str())?,

                    ))
                })
                .collect::<Result<HashMap<_, _>, Box<dyn Error>>>()?,
        };
        Ok(scheme)
    }
}


