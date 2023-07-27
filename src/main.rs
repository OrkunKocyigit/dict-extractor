use crate::command_line::Options;
use clap::Parser;

mod command_line;

fn main() {
    let options = Options::parse();
    println!("Hello, world!");
}
