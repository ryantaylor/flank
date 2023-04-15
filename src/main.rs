use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use vault::Replay;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Flank {
    /// Path to a CoH3 replay file
    file: PathBuf,
}

fn main() -> Result<()> {
    let flank = Flank::parse();

    let file = File::open(flank.file)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let replay = Replay::from_bytes(&buffer).map_err(|_| anyhow!("Parsing failed!"))?;
    let json = serde_json::to_string(&replay)?;

    Ok(println!("{}", json))
}
