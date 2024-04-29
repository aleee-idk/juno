use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use std::path::PathBuf;

use crate::configuration::CONFIG;

pub fn walk_dir(path: &PathBuf) -> Result<Vec<PathBuf>, &str> {
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();

    let accepted_filetypes = ["mp3", "flac"];

    for filetype in accepted_filetypes {
        let _ = types_builder.add("sound", format!("*.{}", filetype).as_str());
    }

    types_builder.select("sound");

    let search_path = CONFIG.base_path.join(path);
    eprintln!(
        "DEBUGPRINT[1]: file_explorer.rs:19: search_path={:#?}",
        search_path
    );

    // PathBuf.join() can override the hole path, this ensure we're not accessing files outside
    // base_dir
    if !search_path.starts_with(&CONFIG.base_path) {
        return Err("Tried to access file or directory outside of server `base_dir` config.");
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
