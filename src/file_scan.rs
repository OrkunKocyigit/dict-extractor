use io::Error;
use rayon::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub fn read_files<P: AsRef<Path>>(paths: &Mutex<Vec<PathBuf>>, path: P) -> Result<(), Error> {
    path.as_ref()
        .read_dir()?
        .filter_map(|e| e.ok())
        .par_bridge()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() && is_extension_valid(&path) {
                paths.lock().unwrap().push(path);
            } else if path.is_dir() {
                read_files(&paths, path).unwrap();
            }
        });
    Ok(())
}

const VALID_EXTENSIONS: [&str; 3] = ["zip", "rar", "7z"];

fn is_extension_valid(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map_or(false, |e| VALID_EXTENSIONS.contains(&e))
}
