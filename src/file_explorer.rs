use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use std::env;
use std::path::PathBuf;

use crate::configuration::{Commands, CONFIG};

pub fn walk_dir(scan_dir: Option<&PathBuf>) -> Result<Vec<PathBuf>, &str> {
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();

    let accepted_filetypes = ["mp3", "flac", "wav"];

    for filetype in accepted_filetypes {
        let _ = types_builder.add("sound", format!("*.{}", filetype).as_str());
    }

    types_builder.select("sound");

    let mut base_path = env::current_dir().expect("Error accesing the enviroment");

    if let Commands::Start {
        base_path: config_path,
    } = &CONFIG.command
    {
        base_path = config_path.to_owned();
    };

    let search_path;

    match scan_dir {
        Some(dir) => {
            search_path = base_path
                .join(dir)
                .canonicalize()
                .expect("Couldn't canonicalizice the path")
        }
        None => search_path = base_path.to_owned(),
    }

    // PathBuf.join() can override the hole path, this ensure we're not accessing files outside
    // base_dir
    if !search_path.starts_with(base_path) {
        return Err("Tried to access file or directory outside of server `base_path` config.");
    }

    let entries: Vec<PathBuf> = WalkBuilder::new(search_path)
        .types(types_builder.build().unwrap())
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path().is_dir())
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(entries)
}
