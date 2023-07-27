use io::Error;
use std::io;
use std::path::{Path, PathBuf};

pub fn read_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Error> {
    let result = path.as_ref().read_dir()?.filter_map(|e| e.ok());
    let mut paths = Vec::new();
    for entry in result {
        let path = entry.path();
        if path.is_file() && VALID_EXTENSIONS.contains(&path.extension().unwrap().to_str().unwrap())
        {
            paths.push(path);
        } else if path.is_dir() {
            paths.extend(read_files(path)?);
        }
    }
    Ok(paths)
}

const VALID_EXTENSIONS: [&str; 3] = ["zip", "rar", "7z"];
