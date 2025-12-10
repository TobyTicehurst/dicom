use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    // output being None indicates that the program output should be written to stdout
    pub output: Option<String>,
    #[arg(short, long, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,
}

impl Cli {
    pub fn from_args() -> Self {
        Cli::parse()
    }
}
