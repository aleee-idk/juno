use std::{env, path::PathBuf};

use clap::Parser;
use std::error::Error;

mod file_explorer;
mod grpc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Directory to scan for files")]
    path: Option<PathBuf>,
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    let path = cli
        .path
        .unwrap_or(env::current_dir().expect("Current directory is not available."));

    let files = file_explorer::walk_dir(&path).expect("error");

    eprintln!("DEBUGPRINT[4]: main.rs:20: files={:#?}", files.len());

    let server = grpc::run()?;

    server.connect().await?;

    Ok(())
}
