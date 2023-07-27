use clap::Parser;
use std::path::PathBuf;

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
}

fn parse_directory(s: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(s);
    if path_buf.exists() && path_buf.is_dir() {
        Ok(path_buf)
    } else {
        Err("Given path is not directory".into())
    }
}
