use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use std::{env, io, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Directory to scan for files")]
    path: Option<PathBuf>,
}

fn walk_dir(path: &PathBuf) -> io::Result<()> {
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();

    let accepted_filetypes = ["mp3", "flac"];

    for filetype in accepted_filetypes {
        let _ = types_builder.add("sound", format!("*.{}", filetype).as_str());
    }

    types_builder.select("sound");

    let entries = WalkBuilder::new(path)
        .types(types_builder.build().unwrap())
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path().is_dir());

    for result in entries {
        let path = result.path();

        println!("{}", path.display());
    }
    Ok(())
}

fn main() {
    let cli = Args::parse();
    let path = cli
        .path
        .unwrap_or(env::current_dir().expect("Current directory is not available."));

    walk_dir(&path).expect("error");
}
