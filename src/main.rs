use std::fs;
use std::process::Command;
use std::sync::{Arc, Mutex};

use clap::Parser;
use rayon::prelude::*;

use crate::command_line::Options;

mod command_line;
mod file_scan;

fn main() -> Result<(), anyhow::Error> {
    let options = Arc::new(Options::parse());
    let paths = Arc::new(Mutex::new(Vec::new()));
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(options.workers())
        .build()
        .unwrap();
    pool.install(|| {
        file_scan::read_files(&paths, options.path()).expect("Creating file list failed");
        paths.lock().unwrap().par_iter().for_each(|path| {
            let parent = path.parent().unwrap();
            let name = path.file_stem().unwrap().to_str().unwrap();
            let target_folder = parent.join(name);
            if target_folder.exists() {
                let _ = fs::remove_dir_all(&target_folder);
            }
            println!("{} extraction started.", &path.display());
            let mut command = Command::new("7z");
            let mut child = command
                .arg("x")
                .arg(&path)
                .arg(format!(
                    "-o{}",
                    target_folder.into_os_string().into_string().unwrap()
                ))
                .arg("-aoa")
                .arg(format!("-p{}", options.password()));
            if let Some(e) = options.encoding() {
                child = command.arg(format!("-mcp={}", e))
            }
            let output = child.output();
            match output {
                Ok(c) => {
                    if c.status.success() {
                        println!("{} has been extracted successfully.", &path.display());
                        if options.delete() {
                            match fs::remove_file(&path) {
                                Ok(_) => println!("{} has been deleted.", &path.display()),
                                _ => println!("{} delete failed", &path.display()),
                            }
                        }
                    } else {
                        println!("{} extracted failed.", &path.display())
                    }
                }
                _ => println!("{} extracted failed.", &path.display()),
            }
        });
    });
    Ok(())
}
