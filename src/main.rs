use std::cmp::max;
use std::fmt::Debug;
use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::thread::available_parallelism;

use clap::Parser;
use threadpool::ThreadPool;

use crate::command_line::Options;

mod command_line;
mod file_scan;

fn main() -> Result<(), anyhow::Error> {
    let options = Arc::new(Options::parse());
    let paths = file_scan::read_files(options.path())?;
    let default_parallelism_approx = available_parallelism().unwrap().get();
    let pool = ThreadPool::new(max(default_parallelism_approx / 2, 1));
    for path in paths {
        let options = Arc::clone(&options);
        pool.execute(move || {
            let parent = path.parent().unwrap();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let target_folder = parent.join(name);
            if target_folder.exists() {
                let _ = fs::remove_dir_all(&target_folder);
            }
            let child = Command::new("7z")
                .arg("x")
                .arg(&path)
                .arg(format!("-mcp={}", options.encoding()))
                .arg(format!(
                    "-o{}",
                    target_folder.into_os_string().into_string().unwrap()
                ))
                .arg("-aoa")
                .arg(format!("-p{}", options.password()))
                .output();
            match child {
                Ok(c) => {
                    if c.status.success() && options.delete() {
                        let _ = fs::remove_file(&path);
                    }
                }
                _ => (),
            }
        });
    }
    pool.join();
    Ok(())
}
