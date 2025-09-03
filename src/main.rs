mod args;

use std::{collections::HashMap, error::Error, io::{Read, Write}};

use args::{Cli, Command};
use clap::Parser;
use regex::Regex;

#[derive(Debug)]
struct Colorset {
    colors: HashMap<String, Color>,
}

impl Colorset {
    fn from_yaml_str(colors_yaml: &str) -> Result<Self, Box<dyn Error>> {
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
        template = regex.replace_all(template.as_str(), to).into_owned();
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut input = String::new();
    cli.input.open()?.read_to_string(&mut input)?;

    let out = match cli.command {
        Command::Fill { to_set } => {
            let mut colors_yaml = String::new();
            to_set.open()?.read_to_string(&mut colors_yaml)?;
            let set = Colorset::from_yaml_str(&colors_yaml)?;

            fill_temaplate(input.as_str(), set)
        }
        Command::Make { from_set } => {
            let mut colors_yaml = String::new();
            from_set.open()?.read_to_string(&mut colors_yaml)?;
            let set = Colorset::from_yaml_str(&colors_yaml)?;

            make_template(input.as_str(), set)
        }
        Command::Transform { from_set, to_set } => {
            let mut colors_yaml = String::new();
            from_set.open()?.read_to_string(&mut colors_yaml)?;
            let from_set = Colorset::from_yaml_str(&colors_yaml)?;

            let mut colors_yaml = String::new();
            to_set.open()?.read_to_string(&mut colors_yaml)?;
            let to_set = Colorset::from_yaml_str(&colors_yaml)?;

            fill_temaplate(make_template(input.as_str(), from_set).as_str(), to_set)
        }
    };

    cli.output.create()?.write(out.as_bytes())?;

    Ok(())
}
