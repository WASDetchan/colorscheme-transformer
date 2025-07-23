use std::{collections::HashMap, error::Error};

use regex::Regex;

#[derive(Debug)]
enum Mode {
    Fill,
    Make,
    Transform,
}

impl Mode {
    fn from_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        Ok(if let Some(mode) = args.next() {
            match mode.as_str() {
                "fill" => Mode::Fill,
                "make" => Mode::Make,
                "transform" => Mode::Transform,
                _ => {
                    return Err(format!("Invalid mode: {mode}. Accepted: fill, make.").into());
                }
            }
        } else {
            return Err("Mode argument not found".into());
        })
    }
}

#[derive(Debug)]
struct Colorset {
    colors: HashMap<String, Color>,
}

impl Colorset {
    fn from_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        let filename = args.next().ok_or("Colorset argument not found")?;
        let colors_yaml = std::fs::read_to_string(filename)
            .map_err(|e| format!("Invalid colorset file: {}", e))?;
        let colors_strings: HashMap<String, String> = serde_yml::from_str(&colors_yaml)
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
struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

impl Color {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
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

    fn to_string(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

fn make_template(colorscheme: &str, set: Colorset) -> String {
    let mut template = colorscheme.to_owned();

    for (name, color) in set.colors.iter() {
        let from = format!("#{}", color.to_string());
        let to = format!("#{{{name}}}");
        let regex = Regex::new(format!("(?i){}", from).as_str()).unwrap();
        template =regex.replace_all(template.as_str(), to).into_owned();
        // template = template.replace(from.as_str(), to.as_str());
    }

    template
}

fn fill_temaplate(template: &str, scheme: Colorset) -> String {
    let mut filled_template = template.to_owned();

    for (name, color) in scheme.colors.iter() {
        let from = format!("{{{name}}}");
        let to = color.to_string();
        filled_template = filled_template.replace(from.as_str(), to.as_str());
    }

    filled_template
}

fn template_from_args(args: &mut impl Iterator<Item = String>) -> Result<String, Box<dyn Error>> {
    let filename = args.next().ok_or("Template argument not found")?;
    let template = std::fs::read_to_string(filename)
        .map_err(|e| format!("Invalid template file: {}", e))?;
    Ok(template)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let mode = Mode::from_args(&mut args)?;


    let out = match mode {
        Mode::Fill => {
            let template = template_from_args(&mut args)?;
            let scheme = Colorset::from_args(&mut args)?;
            fill_temaplate(template.as_str(), scheme)
        }
        Mode::Make => {
            let filled_template = template_from_args(&mut args)?;
            let scheme = Colorset::from_args(&mut args)?;
            make_template(filled_template.as_str(), scheme)
        }
        Mode::Transform => {
            let template_from = template_from_args(&mut args)?;
            let scheme_from = Colorset::from_args(&mut args)?;
            let scheme_to = Colorset::from_args(&mut args)?;
            fill_temaplate(
                make_template(
                    template_from.as_str(), 
                    scheme_from
                ).as_str(),
                scheme_to
            )
        }
    };

    print!("{}", out);

    Ok(())
}
