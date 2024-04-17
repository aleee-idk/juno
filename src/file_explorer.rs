use ignore::types::TypesBuilder;
use ignore::WalkBuilder;
use std::{io, path::PathBuf};

pub fn walk_dir(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut types_builder = TypesBuilder::new();
    types_builder.add_defaults();

    let accepted_filetypes = ["mp3", "flac"];

    for filetype in accepted_filetypes {
        let _ = types_builder.add("sound", format!("*.{}", filetype).as_str());
    }

    types_builder.select("sound");

    let entries: Vec<PathBuf> = WalkBuilder::new(path)
        .types(types_builder.build().unwrap())
        .build()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path().is_dir())
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(entries)
}
