use io::Error;
use std::io;
use std::path::{Path, PathBuf};

pub fn read_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();
    for entry in path.as_ref().read_dir()?.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && is_extension_valid(&path) {
            paths.push(path);
        } else if path.is_dir() {
            paths.extend(read_files(path)?);
        }
    }
    Ok(paths)
}

const VALID_EXTENSIONS: [&str; 3] = ["zip", "rar", "7z"];

fn is_extension_valid(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map_or(false, |e| VALID_EXTENSIONS.contains(&e))
}
