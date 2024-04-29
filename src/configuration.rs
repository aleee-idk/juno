use clap::Parser;
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Directory to scan for files")]
    path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct Config {
    pub base_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_path: env::current_dir().expect("Current directory is not available."),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();

        let cli = Self::get_cli_args();

        if let Some(path) = cli.path {
            config.base_path = path;
        }

        config
    }

    fn get_cli_args() -> Args {
        Args::parse()
    }
}
