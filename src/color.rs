use std::{collections::HashMap, error::Error, fmt::Display};

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
                        Color::from_str(color_string.as_str())?,
                    ))
                })
                .collect::<Result<HashMap<_, _>, Box<dyn Error>>>()?,
        };
        Ok(scheme)
    }
}

#[derive(Debug)]
pub struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

impl Color {
    pub fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        if s.len() != 6 {
            return Err(format!("Invalid color: {}", s).into());
        }
        let chars = s.chars();

        let symbols_int: Vec<u32> = chars
            .map(|c| c.to_digit(16).ok_or(format!("Invalid color: {}", s)))
            .collect::<Result<_, _>>()?;

        Ok(Color {
            red: symbols_int[0] * 16 + symbols_int[1],
            green: symbols_int[2] * 16 + symbols_int[3],
            blue: symbols_int[4] * 16 + symbols_int[5],
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
