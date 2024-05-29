use clap::Parser;
use lazy_static::lazy_static;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use crate::grpc;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Debug)]
pub enum ConfigMode {
    Server,
    Client,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Directory to scan for files")]
    path: Option<PathBuf>,
    #[arg(short, long, help = "the port to bind to", default_value = "50051")]
    port: u16,
    #[arg(
        long,
        help = "The value 1.0 is the “normal” volume. Any value other than 1.0 will multiply each sample by this value.",
        default_value = "1.0"
    )]
    volume: f32,
}

#[derive(Debug)]
pub struct Config {
    pub base_path: PathBuf,
    pub address: SocketAddr,
    pub mode: ConfigMode,
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            base_path: env::current_dir().expect("Current directory is not available."),
            mode: ConfigMode::Server,
            address: SocketAddr::from_str("[::1]:50051").unwrap(),
            volume: 1.0,
        }
    }
}
impl Config {
    pub fn new() -> Self {
        let cli = Self::get_cli_args();

        let mut config = Self::default();
        config.address = SocketAddr::from_str(format!("[::1]:{}", cli.port).as_str()).unwrap();
        config.volume = cli.volume;

        if let Some(path) = cli.path {
            config.base_path = path;
        }

        if grpc::is_socket_in_use(config.address) {
            config.mode = ConfigMode::Client;
        } else {
            config.mode = ConfigMode::Server;
        };

        config
    }

    fn get_cli_args() -> Args {
        Args::parse()
    }
}
