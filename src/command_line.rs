use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[arg(value_parser = parse_directory)]
    path: PathBuf,
    #[arg(short, long, default_value = "false")]
    delete: bool,
    #[arg(short, long, default_value = "nowaythisis")]
    password: String,
    #[arg(short, long, default_value = "932")]
    encoding: String,
    #[arg(hide = true, default_value = "7z", value_parser = check_archiver)]
    archiver: String,
}

impl Options {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn delete(&self) -> bool {
        self.delete
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn encoding(&self) -> &str {
        &self.encoding
    }
}

fn parse_directory(s: &str) -> Result<PathBuf, String> {
    match dunce::canonicalize(PathBuf::from(s)) {
        Ok(p) => {
            if p.exists() && p.is_dir() {
                Ok(p)
            } else {
                Err("Given path is not directory".into())
            }
        }
        _ => Err("Given path is not directory".into()),
    }
}

fn check_archiver(s: &str) -> Result<String, String> {
    if find_executable(s) {
        Ok(s.to_string())
    } else {
        Err(format!(
            "{} has to be installed and in path in order to use this program",
            s
        ))
    }
}

fn find_executable(name: &str) -> bool {
    Command::new(name).stdout(Stdio::null()).spawn().is_ok()
}
