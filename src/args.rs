use clio::{InputPath, OutputPath};

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(long, short, value_parser, default_value = "-", global = true)]
    pub input: InputPath,
    #[clap(long, short, value_parser, default_value = "-", global = true)]
    pub output: OutputPath,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Fill {
        #[clap(value_parser)]
        to_set: InputPath,
    },
    Make {
        #[clap(value_parser)]
        from_set: InputPath,
    },
    Transform {
        #[clap(value_parser)]
        from_set: InputPath,
        #[clap(value_parser)]
        to_set: InputPath,
    },
}
