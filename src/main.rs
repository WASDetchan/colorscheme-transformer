mod args;
mod color;

use std::{
    error::Error,
    io::{Read, Write},
};

use args::{Cli, Command};
use clap::Parser;
use color::{to_format, ColorFormat};
use regex::Regex;


use crate::color::Colorset;

fn make_template(colorscheme: &str, set: Colorset) -> String {
    let mut template = colorscheme.to_owned();

    for (name, color) in set.colors.iter() {
        let from = format!("{}", to_format(color, ColorFormat::Hex));
        let to = format!("#{{{name}}}");
        let regex = Regex::new(format!("(?i){}", from).as_str()).unwrap();
        template = regex.replace_all(template.as_str(), to).into_owned();
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
            eprintln!("{}; {}", from_set, to_set);

            let mut colors_yaml = String::new();
            from_set.open()?.read_to_string(&mut colors_yaml)?;
            let from_set = Colorset::from_yaml_str(&colors_yaml)?;

            let mut colors_yaml = String::new();
            to_set.open()?.read_to_string(&mut colors_yaml)?;
            let to_set = Colorset::from_yaml_str(&colors_yaml)?;

            fill_temaplate(make_template(input.as_str(), from_set).as_str(), to_set)
        }
    };

    let amount = cli.output.create()?.write(out.as_bytes())?;

    eprint!("Written amount: {}", amount);

    Ok(())
}
