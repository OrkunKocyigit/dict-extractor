use clap::Parser;

use crate::command_line::Options;

mod command_line;
mod file_scan;

fn main() -> Result<(), anyhow::Error> {
    let options = Options::parse();
    let paths = file_scan::read_files(options.path())?;
    println!("{:?}", paths);
    Ok(())
}
