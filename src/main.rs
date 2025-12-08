mod day1;

use clap::Parser;
use day1::{d1p1};
use std::{fs, error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    part: u8,
    #[arg(short, long)]
    input_file: Option<String>
}

const INPUT_FILES: [[&str;2]; 1] = [
    ["d1p1", "d1p2"]
];
const INPUT_DIR: &str = "./inputs";

fn read_input_file(day: u8, part: u8) -> Result<String, Box<dyn Error>> {
    let filename: &str = INPUT_FILES[(day - 1) as usize][(part - 1) as usize];
    Ok(fs::read_to_string(PathBuf::from(INPUT_DIR).join(filename))?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let msg: String = match args.input_file {
        Some(x) => fs::read_to_string(x)?,
        None => read_input_file(args.day, args.part)?,
    };

    match (args.day, args.part) {
        (1, 1) => {
            println!("{}", d1p1(msg)?);
        }
        _ => {
            println!("day {} part {} is not available!", args.day, args.part);
        }
    }
    Ok(())
}
