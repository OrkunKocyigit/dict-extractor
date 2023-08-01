use std::cmp::max;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread::available_parallelism;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[arg(value_parser = parse_directory)]
    path: PathBuf,
    #[arg(short, long, default_value = "false")]
    delete: bool,
    #[arg(short, long, default_value = "nowaythisis")]
    password: String,
    #[arg(long, default_value_t = false)]
    no_encoding: bool,
    #[arg(
        short,
        long,
        default_value_if("no_encoding", "true", None),
        default_value_if("no_encoding", "false", Some("932"))
    )]
    encoding: Option<String>,
    #[arg(short, long, default_value_t = get_thread_num(), value_parser = parse_thread_count)]
    workers: usize,
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
    pub fn encoding(&self) -> &Option<String> {
        &self.encoding
    }
    pub fn workers(&self) -> usize {
        self.workers
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

fn parse_thread_count(count: &str) -> Result<usize, String> {
    let c = count
        .parse::<usize>()
        .map_err(|_| "Worker parameter is not valid number.".to_string())?;
    if c < 1 {
        Err("Worker parameter cannot be lower than 1".into())
    } else {
        Ok(c)
    }
}

fn get_thread_num() -> usize {
    let default_parallelism_approx = available_parallelism().unwrap().get();
    max(default_parallelism_approx / 2, 1)
}

#[test]
fn test_options_when_no_encoding_present() {
    let p = Options::command().get_matches_from(vec!["", "--no-encoding", "./test"]);
    assert_eq!(p.get_one::<String>("encoding"), None);
}

#[test]
fn test_options_when_no_encoding_not_present() {
    let p = Options::command().get_matches_from(vec!["", "./test"]);
    assert_eq!(p.get_one::<String>("encoding"), Some(&"932".to_string()));
}

#[test]
fn test_options_when_custom_encoding_present() {
    let p = Options::command().get_matches_from(vec!["", "-e", "123", "./test"]);
    assert_eq!(p.get_one::<String>("encoding"), Some(&"123".to_string()));
}
