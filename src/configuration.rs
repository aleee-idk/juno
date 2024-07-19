use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use crate::grpc;

#[derive(Debug)]
pub enum ConfigMode {
    Server,
    Client,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Start the GRPC server
    Start {
        #[arg(help = "Directory to scan for files", default_value = ".")]
        base_path: PathBuf,
        #[arg(
            long,
            help = "The value 1.0 is the “normal” volume. Any value other than 1.0 will multiply each sample by this value.",
            default_value = "1.0"
        )]
        volume: f32,
    },
    /// Resume the playback
    Play,
    /// Pause the playback
    Pause,
    /// Resume the playback if pause, pause if is playing
    PlayPause,
    /// Skip the current song
    SkipSong,
    Set,
    /// List the available files
    GetFiles {
        #[arg(
            help = "Directory to scan for files, relative to server base path",
            default_value = "."
        )]
        path: PathBuf,
    },
    /// Test server connection
    Ping,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,

    #[arg(short, long, help = "the port to bind to", default_value = "50051")]
    port: u16,
}

#[derive(Debug)]
pub struct Config {
    pub command: Commands,
    pub address: SocketAddr,
    pub mode: ConfigMode,
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            command: Commands::Play,
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
        config.command = cli.cmd;

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
